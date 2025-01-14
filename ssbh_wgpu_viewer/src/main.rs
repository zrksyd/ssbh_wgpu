use pico_args::Arguments;
use ssbh_data::prelude::*;
use ssbh_wgpu::animation::camera::animate_camera;
use ssbh_wgpu::next_frame;
use ssbh_wgpu::swing::SwingPrc;
use ssbh_wgpu::viewport::screen_to_world;
use ssbh_wgpu::CameraTransforms;
use ssbh_wgpu::DebugMode;
use ssbh_wgpu::ModelFolder;
use ssbh_wgpu::ModelRenderOptions;
use ssbh_wgpu::NutexbFile;
use ssbh_wgpu::RenderModel;
use ssbh_wgpu::RenderSettings;
use ssbh_wgpu::SharedRenderData;
use ssbh_wgpu::TransitionMaterial;
use ssbh_wgpu::REQUIRED_FEATURES;
use ssbh_wgpu::{load_model_folders, load_render_models, SsbhRenderer};
use std::collections::HashSet;
use std::path::PathBuf;
use winit::{
    dpi::PhysicalPosition,
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

const DEFAULT_FOV: f32 = 0.5;
const DEFAULT_NEAR_CLIP: f32 = 1.0;
const DEFAULT_FAR_CLIP: f32 = 400000.0;

fn calculate_camera_pos_mvp(
    size: winit::dpi::PhysicalSize<u32>,
    translation: glam::Vec3,
    rotation: glam::Vec3,
) -> (glam::Vec4, glam::Mat4, glam::Mat4) {
    let aspect = size.width as f32 / size.height as f32;
    let model_view_matrix = glam::Mat4::from_translation(translation)
        * glam::Mat4::from_rotation_x(rotation.x)
        * glam::Mat4::from_rotation_y(rotation.y);
    // Use a large far clip distance to include stage skyboxes.
    let perspective_matrix =
        glam::Mat4::perspective_rh(DEFAULT_FOV, aspect, DEFAULT_NEAR_CLIP, DEFAULT_FAR_CLIP);

    let camera_pos = model_view_matrix.inverse().col(3);

    (
        camera_pos,
        model_view_matrix,
        perspective_matrix * model_view_matrix,
    )
}

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,

    // Parallel lists for models and renderable models.
    models: Vec<(PathBuf, ModelFolder)>,
    render_models: Vec<RenderModel>,

    renderer: SsbhRenderer,

    // TODO: Separate camera/window state struct?
    size: winit::dpi::PhysicalSize<u32>,

    // Camera input stuff.
    previous_cursor_position: PhysicalPosition<f64>,
    is_mouse_left_clicked: bool,
    is_mouse_right_clicked: bool,
    translation_xyz: glam::Vec3,
    rotation_xyz: glam::Vec3,

    // Animations
    animation: Option<AnimData>,
    camera_animation: Option<AnimData>,
    light_animation: Option<AnimData>,

    // TODO: How to handle overflow if left running too long?
    current_frame: f32,
    previous_frame_start: std::time::Instant,

    // TODO: Should this be part of the renderer?
    shared_data: SharedRenderData,

    is_playing: bool,

    render: RenderSettings,
}

impl State {
    async fn new(
        window: &Window,
        folder: PathBuf,
        anim: Option<PathBuf>,
        prc: Option<PathBuf>,
        camera_anim: Option<PathBuf>,
        render_folder: Option<PathBuf>,
    ) -> Self {
        // TODO: Investigate DX12 errors on Windows.
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        let surface = unsafe { instance.create_surface(window).unwrap() };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::default() | REQUIRED_FEATURES,
                    limits: wgpu::Limits::default(),
                },
                None, // Trace path
            )
            .await
            .unwrap();

        let size = window.inner_size();

        let surface_format = ssbh_wgpu::RGBA_COLOR_FORMAT;

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: Vec::new(),
        };
        surface.configure(&device, &config);

        // TODO: Frame bounding spheres?
        let animation = anim.map(|anim_path| AnimData::from_file(anim_path).unwrap());
        let swing_prc = prc.and_then(|prc_path| SwingPrc::from_file(prc_path));
        let camera_animation =
            camera_anim.map(|camera_anim_path| AnimData::from_file(camera_anim_path).unwrap());

        // Try different possible paths.
        let light_animation = render_folder
            .as_ref()
            .and_then(|f| AnimData::from_file(f.join("light").join("light00.nuanmb")).ok())
            .or_else(|| {
                render_folder
                    .as_ref()
                    .and_then(|f| AnimData::from_file(f.join("light").join("light_00.nuanmb")).ok())
            });

        let mut shared_data = SharedRenderData::new(&device, &queue, surface_format);

        // Update the cube map first since it's used in model loading for texture assignments.
        if let Some(nutexb) = render_folder
            .as_ref()
            .and_then(|f| NutexbFile::read_from_file(f.join("reflection_cubemap.nutexb")).ok())
        {
            shared_data.update_stage_cube_map(&device, &queue, &nutexb);
        }

        let models = load_model_folders(folder);
        let mut render_models =
            load_render_models(&device, &queue, models.iter().map(|(_, m)| m), &shared_data);

        // Assume only one folder is loaded and apply the swing prc to every folder.
        if let Some(swing_prc) = &swing_prc {
            for (render_model, (_, model)) in render_models.iter_mut().zip(models.iter()) {
                render_model.recreate_swing_collisions(&device, swing_prc, model.find_skel());
            }
        }

        let mut renderer = SsbhRenderer::new(
            &device,
            &queue,
            size.width,
            size.height,
            window.scale_factor(),
            [0.0; 3],
            &[],
        );

        if let Some(nutexb) = render_folder.as_ref().and_then(|f| {
            NutexbFile::read_from_file(
                f.parent()
                    .unwrap()
                    .join("lut")
                    .join("color_grading_lut.nutexb"),
            )
            .ok()
        }) {
            renderer.update_color_lut(&device, &queue, &nutexb);
        }

        Self {
            surface,
            device,
            queue,
            config,
            size,
            models,
            render_models,
            renderer,
            previous_cursor_position: PhysicalPosition { x: 0.0, y: 0.0 },
            is_mouse_left_clicked: false,
            is_mouse_right_clicked: false,
            translation_xyz: glam::vec3(0.0, -8.0, -60.0),
            rotation_xyz: glam::vec3(0.0, 0.0, 0.0),
            animation,
            camera_animation,
            light_animation,
            current_frame: 0.0,
            previous_frame_start: std::time::Instant::now(),
            shared_data,
            is_playing: false,
            render: RenderSettings::default(),
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>, scale_factor: f64) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);

            self.update_camera(scale_factor);

            // We also need to recreate the attachments if the size changes.
            self.renderer.resize(
                &self.device,
                &self.queue,
                new_size.width,
                new_size.height,
                scale_factor,
                [0, 0, new_size.width, new_size.height],
            );
        }
    }

    fn handle_input(&mut self, event: &WindowEvent) -> bool {
        // Return true if this function handled the event.
        // TODO: Input handling can be it's own module with proper tests.
        // Just test if the WindowEvent object is handled correctly.
        // Test that some_fn(event, state) returns new state?
        match event {
            WindowEvent::MouseInput { button, state, .. } => {
                // Track mouse clicks to only rotate when dragging while clicked.
                match (button, state) {
                    (MouseButton::Left, ElementState::Pressed) => self.is_mouse_left_clicked = true,
                    (MouseButton::Left, ElementState::Released) => {
                        self.is_mouse_left_clicked = false
                    }
                    (MouseButton::Right, ElementState::Pressed) => {
                        self.is_mouse_right_clicked = true
                    }
                    (MouseButton::Right, ElementState::Released) => {
                        self.is_mouse_right_clicked = false
                    }
                    _ => (),
                }
                true
            }
            WindowEvent::CursorMoved { position, .. } => {
                if self.is_mouse_left_clicked {
                    let delta_x = position.x - self.previous_cursor_position.x;
                    let delta_y = position.y - self.previous_cursor_position.y;

                    // Swap XY so that dragging left right rotates left right.
                    self.rotation_xyz.x += (delta_y * 0.01) as f32;
                    self.rotation_xyz.y += (delta_x * 0.01) as f32;
                } else if self.is_mouse_right_clicked {
                    // TODO: Avoid recalculating the matrix?
                    // TODO: Does ignoring rotation like this work in general?
                    let (_, _, mvp) = calculate_camera_pos_mvp(
                        self.size,
                        self.translation_xyz,
                        self.rotation_xyz * 0.0,
                    );

                    let (current_x_world, current_y_world) = screen_to_world(
                        (position.x as f32, position.y as f32),
                        mvp,
                        self.size.width,
                        self.size.height,
                    );
                    let (previous_x_world, previous_y_world) = screen_to_world(
                        (
                            self.previous_cursor_position.x as f32,
                            self.previous_cursor_position.y as f32,
                        ),
                        mvp,
                        self.size.width,
                        self.size.height,
                    );

                    let delta_x_world = current_x_world - previous_x_world;
                    let delta_y_world = current_y_world - previous_y_world;

                    // Negate y so that dragging up "drags" the model up.
                    self.translation_xyz.x += delta_x_world;
                    self.translation_xyz.y -= delta_y_world;
                }
                // Always update the position to avoid jumps when moving between clicks.
                self.previous_cursor_position = *position;

                true
            }
            WindowEvent::MouseWheel { delta, .. } => {
                // TODO: Add tests for handling scroll events properly?
                // Scale zoom speed with distance to make it easier to zoom out large scenes.
                let delta_z = match delta {
                    MouseScrollDelta::LineDelta(_x, y) => *y * self.translation_xyz.z.abs() * 0.1,
                    MouseScrollDelta::PixelDelta(p) => {
                        p.y as f32 * self.translation_xyz.z.abs() * 0.005
                    }
                };

                // Clamp to prevent the user from zooming through the origin.
                self.translation_xyz.z = (self.translation_xyz.z + delta_z).min(-1.0);
                true
            }
            WindowEvent::KeyboardInput { input, .. } => {
                if let Some(keycode) = input.virtual_keycode {
                    // Don't handle the release event to avoid duplicate events.
                    if matches!(input.state, winit::event::ElementState::Pressed) {
                        match keycode {
                            VirtualKeyCode::Up => self.translation_xyz.z += 10.0,
                            VirtualKeyCode::Down => self.translation_xyz.z -= 10.0,
                            VirtualKeyCode::Space => self.is_playing = !self.is_playing,
                            VirtualKeyCode::Key1 => self.render.debug_mode = DebugMode::Shaded,
                            VirtualKeyCode::Key2 => self.render.debug_mode = DebugMode::ColorSet1,
                            VirtualKeyCode::Key3 => self.render.debug_mode = DebugMode::ColorSet2,
                            VirtualKeyCode::Key4 => self.render.debug_mode = DebugMode::ColorSet3,
                            VirtualKeyCode::Key5 => self.render.debug_mode = DebugMode::ColorSet4,
                            VirtualKeyCode::Key6 => self.render.debug_mode = DebugMode::ColorSet5,
                            VirtualKeyCode::Key7 => self.render.debug_mode = DebugMode::ColorSet6,
                            VirtualKeyCode::Key8 => self.render.debug_mode = DebugMode::ColorSet7,
                            VirtualKeyCode::Q => self.render.debug_mode = DebugMode::Texture0,
                            VirtualKeyCode::W => self.render.debug_mode = DebugMode::Texture1,
                            VirtualKeyCode::E => self.render.debug_mode = DebugMode::Texture2,
                            VirtualKeyCode::R => self.render.debug_mode = DebugMode::Texture3,
                            VirtualKeyCode::T => self.render.debug_mode = DebugMode::Texture4,
                            VirtualKeyCode::Y => self.render.debug_mode = DebugMode::Texture5,
                            VirtualKeyCode::U => self.render.debug_mode = DebugMode::Texture6,
                            VirtualKeyCode::I => self.render.debug_mode = DebugMode::Texture7,
                            VirtualKeyCode::O => self.render.debug_mode = DebugMode::Texture8,
                            VirtualKeyCode::P => self.render.debug_mode = DebugMode::Texture9,
                            VirtualKeyCode::A => self.render.debug_mode = DebugMode::Texture10,
                            VirtualKeyCode::S => self.render.debug_mode = DebugMode::Texture11,
                            VirtualKeyCode::D => self.render.debug_mode = DebugMode::Texture12,
                            VirtualKeyCode::F => self.render.debug_mode = DebugMode::Texture13,
                            VirtualKeyCode::G => self.render.debug_mode = DebugMode::Texture14,
                            VirtualKeyCode::H => self.render.debug_mode = DebugMode::Texture16,
                            VirtualKeyCode::J => self.render.debug_mode = DebugMode::Position0,
                            VirtualKeyCode::K => self.render.debug_mode = DebugMode::Normal0,
                            VirtualKeyCode::L => self.render.debug_mode = DebugMode::Tangent0,
                            VirtualKeyCode::Z => self.render.debug_mode = DebugMode::Map1,
                            VirtualKeyCode::X => self.render.debug_mode = DebugMode::Bake1,
                            VirtualKeyCode::C => self.render.debug_mode = DebugMode::UvSet,
                            VirtualKeyCode::V => self.render.debug_mode = DebugMode::UvSet1,
                            VirtualKeyCode::B => self.render.debug_mode = DebugMode::UvSet2,
                            VirtualKeyCode::N => self.render.debug_mode = DebugMode::Basic,
                            VirtualKeyCode::M => self.render.debug_mode = DebugMode::Normals,
                            VirtualKeyCode::Comma => self.render.debug_mode = DebugMode::Bitangents,
                            VirtualKeyCode::Period => self.render.debug_mode = DebugMode::Unlit,
                            VirtualKeyCode::Slash => {
                                self.render.debug_mode = DebugMode::ShaderComplexity
                            }
                            // TODO: Add more steps?
                            VirtualKeyCode::Numpad0 => self.render.transition_factor = 0.0,
                            VirtualKeyCode::Numpad1 => self.render.transition_factor = 1.0 / 3.0,
                            VirtualKeyCode::Numpad2 => self.render.transition_factor = 2.0 / 3.0,
                            VirtualKeyCode::Numpad3 => self.render.transition_factor = 1.0,
                            VirtualKeyCode::Numpad4 => {
                                self.render.transition_material = TransitionMaterial::Ink
                            }
                            VirtualKeyCode::Numpad5 => {
                                self.render.transition_material = TransitionMaterial::MetalBox
                            }
                            VirtualKeyCode::Numpad6 => {
                                self.render.transition_material = TransitionMaterial::Gold
                            }
                            VirtualKeyCode::Numpad7 => {
                                self.render.transition_material = TransitionMaterial::Ditto
                            }
                            _ => (),
                        }
                    }
                }

                true
            }
            _ => false,
        }
    }

    // TODO: Module and tests for a viewport camera.

    fn update_camera(&mut self, scale_factor: f64) {
        let (camera_pos, model_view_matrix, mvp_matrix) =
            calculate_camera_pos_mvp(self.size, self.translation_xyz, self.rotation_xyz);
        let transforms = CameraTransforms {
            model_view_matrix,
            mvp_matrix,
            mvp_inv_matrix: mvp_matrix.inverse(),
            camera_pos,
            screen_dimensions: glam::vec4(
                self.size.width as f32,
                self.size.height as f32,
                scale_factor as f32,
                0.0,
            ),
        };
        self.renderer.update_camera(&self.queue, transforms);
    }

    fn update_render_settings(&mut self) {
        self.renderer
            .update_render_settings(&self.queue, &self.render);
    }

    fn render(&mut self, scale_factor: f64) -> Result<(), wgpu::SurfaceError> {
        let current_frame_start = std::time::Instant::now();
        if self.is_playing {
            self.current_frame = next_frame(
                self.current_frame,
                current_frame_start.duration_since(self.previous_frame_start),
                self.animation
                    .as_ref()
                    .map(|a| a.final_frame_index)
                    .unwrap_or_default()
                    .max(
                        self.camera_animation
                            .as_ref()
                            .map(|a| a.final_frame_index)
                            .unwrap_or_default(),
                    )
                    .max(
                        self.light_animation
                            .as_ref()
                            .map(|a| a.final_frame_index)
                            .unwrap_or_default(),
                    ),
                1.0,
                true,
            );
        }
        self.previous_frame_start = current_frame_start;

        // Bind groups are preconfigured outside the render loop for performance.
        // This means only the output view needs to be set for each pass.
        let output = self.surface.get_current_texture()?;
        let output_view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        // Apply animations for each model.
        // This is more efficient than animating per mesh since state is shared between render meshes.
        if self.is_playing {
            // TODO: Combine these into one list?
            for (i, model) in self.render_models.iter_mut().enumerate() {
                model.apply_anims(
                    &self.queue,
                    self.animation.iter(),
                    self.models[i].1.find_skel(),
                    self.models[i].1.find_matl(),
                    self.models[i].1.find_hlpb(),
                    &self.shared_data,
                    self.current_frame,
                );
            }

            if let Some(anim) = &self.camera_animation {
                let aspect = self.size.width as f32 / self.size.height as f32;
                let screen_dimensions = glam::vec4(
                    self.size.width as f32,
                    self.size.height as f32,
                    scale_factor as f32,
                    0.0,
                );

                if let Some(transforms) = animate_camera(
                    anim,
                    self.current_frame,
                    aspect,
                    screen_dimensions,
                    DEFAULT_FOV,
                    DEFAULT_NEAR_CLIP,
                    DEFAULT_FAR_CLIP,
                ) {
                    self.renderer.update_camera(&self.queue, transforms);
                }
            }

            if let Some(anim) = &self.light_animation {
                self.renderer
                    .update_stage_uniforms(&self.queue, anim, self.current_frame);
            }
        }

        let mut final_pass = self.renderer.render_models(
            &mut encoder,
            &output_view,
            &self.render_models,
            self.shared_data.database(),
            &ModelRenderOptions {
                draw_bones: false,
                draw_bone_axes: false,
                draw_floor_grid: false,
                ..Default::default()
            },
        );

        for model in &self.render_models {
            // Use an empty set to show all collisions.
            self.renderer
                .render_swing(&mut final_pass, model, &HashSet::new());
        }

        drop(final_pass);

        let (_, _, mvp) =
            calculate_camera_pos_mvp(self.size, self.translation_xyz, self.rotation_xyz);

        if let Some(text_commands) = self.renderer.render_skeleton_names(
            &self.device,
            &self.queue,
            &output_view,
            self.render_models
                .iter()
                .zip(self.models.iter().map(|(_, m)| m.find_skel()))
                .filter_map(|(m, s)| Some((m, s?))),
            self.size.width,
            self.size.height,
            mvp,
            16.0,
        ) {
            self.queue.submit([encoder.finish(), text_commands]);
        } else {
            self.queue.submit([encoder.finish()]);
        }

        // Actually draw the frame.
        output.present();

        Ok(())
    }
}

fn main() {
    // Ignore most wgpu logs to avoid flooding the console.
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Warn)
        .with_module_level("ssbh_wgpu", log::LevelFilter::Info)
        .init()
        .unwrap();

    let mut args = Arguments::from_env();
    // TODO: Support loading multiple folders.
    let folder: PathBuf = args.free_from_str().unwrap();
    let anim_path: Option<PathBuf> = args.opt_value_from_str("--anim").unwrap();
    let prc_path: Option<PathBuf> = args.opt_value_from_str("--swing").unwrap();
    let camera_anim_path: Option<PathBuf> = args.opt_value_from_str("--camera-anim").unwrap();
    let render_folder_path: Option<PathBuf> = args.opt_value_from_str("--render-folder").unwrap();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("ssbh_wgpu")
        .build(&event_loop)
        .unwrap();

    let mut state = futures::executor::block_on(State::new(
        &window,
        folder,
        anim_path,
        prc_path,
        camera_anim_path,
        render_folder_path,
    ));

    // Initialize the camera buffer.
    state.update_camera(window.scale_factor());

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(_) => {
                        // Use the window size to avoid a potential error from size mismatches.
                        state.resize(window.inner_size(), window.scale_factor());
                    }
                    WindowEvent::ScaleFactorChanged {
                        scale_factor,
                        new_inner_size,
                    } => {
                        // new_inner_size is &mut so we have to dereference it twice
                        state.resize(**new_inner_size, *scale_factor);
                    }
                    _ => {}
                }

                if state.handle_input(event) {
                    // TODO: Avoid overriding the camera values when pausing?
                    state.update_camera(window.scale_factor());

                    state.update_render_settings();
                }
            }
            Event::RedrawRequested(_) => {
                match state.render(window.scale_factor()) {
                    Ok(_) => {}
                    // Recreate the swap_chain if lost
                    Err(wgpu::SurfaceError::Lost) => {
                        state.resize(state.size, window.scale_factor())
                    }
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                window.request_redraw();
            }
            _ => {}
        }
    });
}
