use std::{num::NonZeroU32, path::Path};

use futures::executor::block_on;
use image::ImageBuffer;
use ssbh_wgpu::{
    load_render_models, CameraTransforms, ModelFolder, SharedRenderData, SsbhRenderer,
    REQUIRED_FEATURES, RGBA_COLOR_FORMAT,
};
use wgpu::{
    Backends, DeviceDescriptor, Extent3d, Instance, Limits, PowerPreference, RequestAdapterOptions,
    TextureDescriptor, TextureDimension, TextureUsages,
};

fn calculate_camera_pos_mvp(
    translation: glam::Vec3,
    rotation: glam::Vec3,
) -> (glam::Vec4, glam::Mat4, glam::Mat4) {
    let aspect = 1.0;
    let model_view_matrix = glam::Mat4::from_translation(translation)
        * glam::Mat4::from_rotation_x(rotation.x)
        * glam::Mat4::from_rotation_y(rotation.y);
    // Use a large far clip distance to include stage skyboxes.
    let perspective_matrix = glam::Mat4::perspective_rh(0.5, aspect, 1.0, 400000.0);

    let camera_pos = model_view_matrix.inverse().col(3);

    (
        camera_pos,
        model_view_matrix,
        perspective_matrix * model_view_matrix,
    )
}

fn main() {
    // Check for any errors.
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Warn)
        .with_module_level("ssbh_wgpu", log::LevelFilter::Info)
        .init()
        .unwrap();

    // Load models in headless mode without a surface.
    // This simplifies testing for stability and performance.
    let instance = Instance::new(Backends::all());
    let adapter = block_on(instance.request_adapter(&RequestAdapterOptions {
        power_preference: PowerPreference::HighPerformance,
        compatible_surface: None,
        force_fallback_adapter: false,
    }))
    .unwrap();
    let (device, queue) = block_on(adapter.request_device(
        &DeviceDescriptor {
            label: None,
            features: REQUIRED_FEATURES,
            limits: Limits::default(),
        },
        None,
    ))
    .unwrap();

    // TODO: Find a way to simplify initialization.
    let surface_format = RGBA_COLOR_FORMAT;
    let shared_data = SharedRenderData::new(&device, &queue, surface_format);
    let mut renderer = SsbhRenderer::new(&device, &queue, 512, 512, 1.0, [0.0; 3], &[]);

    // TODO: Share camera code with ssbh_wgpu?
    // TODO: Document the screen_dimensions struct.
    // TODO: Frame each model individually?
    let (camera_pos, model_view_matrix, mvp_matrix) =
        calculate_camera_pos_mvp(glam::Vec3::new(0.0, -8.0, -60.0), glam::Vec3::ZERO);
    let transforms = CameraTransforms {
        model_view_matrix,
        mvp_matrix,
        camera_pos: camera_pos.to_array(),
        screen_dimensions: [512.0, 512.0, 1.0, 0.0],
    };
    renderer.update_camera(&queue, transforms);

    let texture_desc = TextureDescriptor {
        size: Extent3d {
            width: 512,
            height: 512,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,
        format: surface_format,
        usage: TextureUsages::COPY_SRC | TextureUsages::RENDER_ATTACHMENT,
        label: None,
    };
    let output = device.create_texture(&texture_desc);
    let output_view = output.create_view(&Default::default());

    let output_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        size: 512 * 512 * 4,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        label: None,
        mapped_at_creation: false,
    });

    let args: Vec<_> = std::env::args().collect();

    // Load and render folders individually to save on memory.
    let source_folder = Path::new(&args[1]);
    let model_paths = globwalk::GlobWalkerBuilder::from_patterns(source_folder, &["*.{numshb}"])
        .build()
        .unwrap()
        .into_iter()
        .filter_map(Result::ok);

    let start = std::time::Instant::now();
    for model in model_paths.into_iter().filter_map(|p| {
        let parent = p.path().parent()?;
        Some(ModelFolder::load_folder(parent))
    }) {
        // Convert fighter/mario/model/body/c00 to mario_model_body_c00.
        let output_path = Path::new(&model.folder_name)
            .strip_prefix(source_folder)
            .unwrap()
            .components()
            .into_iter()
            .map(|c| c.as_os_str().to_string_lossy())
            .collect::<Vec<_>>()
            .join("_");
        let output_path = source_folder.join(output_path).with_extension("png");
        println!("{:?}", output_path);

        let render_models = load_render_models(&device, &queue, &[model], &shared_data);

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        renderer.render_models(
            &mut encoder,
            &output_view,
            &render_models,
            &shared_data.database,
        );

        encoder.copy_texture_to_buffer(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &output,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            wgpu::ImageCopyBuffer {
                buffer: &output_buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: NonZeroU32::new(512 * 4),
                    rows_per_image: NonZeroU32::new(512),
                },
            },
            texture_desc.size,
        );

        queue.submit([encoder.finish()]);

        // TODO: Move this functionality to ssbh_wgpu for taking screenshots?
        // Save the output texture.
        // Adapted from WGPU Example https://github.com/gfx-rs/wgpu/tree/master/wgpu/examples/capture
        {
            // TODO: Find ways to optimize this?
            let buffer_slice = output_buffer.slice(..);

            // TODO: Reuse the channel?
            let (tx, rx) = futures_intrusive::channel::shared::oneshot_channel();
            buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
                tx.send(result).unwrap();
            });
            device.poll(wgpu::Maintain::Wait);
            block_on(rx.receive()).unwrap().unwrap();

            let data = buffer_slice.get_mapped_range();
            let mut buffer =
                ImageBuffer::<image::Rgba<u8>, _>::from_raw(512, 512, data.to_owned()).unwrap();
            // Convert BGRA to RGBA.
            buffer.pixels_mut().for_each(|p| p.0.swap(0, 2));

            buffer.save(output_path).unwrap();
        }
        output_buffer.unmap();
    }

    println!("Completed in {:?}", start.elapsed());
}
