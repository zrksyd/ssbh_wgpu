// File automatically generated by build.rs.
// Changes made to this file will not be saved.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraTransforms {
    pub model_view_matrix: glam::Mat4,
    pub mvp_matrix: glam::Mat4,
    pub camera_pos: [f32; 4],
    pub screen_dimensions: [f32; 4],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightTransforms {
    pub light_transform: glam::Mat4,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RenderSettings {
    pub debug_mode: [u32; 4],
    pub transition_material: [u32; 4],
    pub transition_factor: [f32; 4],
    pub render_diffuse: [u32; 4],
    pub render_specular: [u32; 4],
    pub render_emission: [u32; 4],
    pub render_rim_lighting: [u32; 4],
    pub render_shadows: [u32; 4],
    pub render_bloom: [u32; 4],
    pub render_vertex_color: [u32; 4],
    pub render_rgba: [f32; 4],
    pub render_nor: [u32; 4],
    pub render_prm: [u32; 4],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Light {
    pub color: [f32; 4],
    pub direction: [f32; 4],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SceneAttributesForShaderFx {
    pub custom_boolean: [[u32; 4]; 20],
    pub custom_vector: [[f32; 4]; 64],
    pub custom_float: [[f32; 4]; 20],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct StageUniforms {
    pub light_chr: Light,
    pub scene_attributes: SceneAttributesForShaderFx,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MaterialUniforms {
    pub custom_vector: [[f32; 4]; 64],
    pub custom_boolean: [[u32; 4]; 20],
    pub custom_float: [[f32; 4]; 20],
    pub has_boolean: [[u32; 4]; 20],
    pub has_float: [[u32; 4]; 20],
    pub has_texture: [[u32; 4]; 19],
    pub has_vector: [[u32; 4]; 64],
    pub has_color_set1234: [u32; 4],
    pub has_color_set567: [u32; 4],
    pub is_discard: [u32; 4],
    pub enable_specular: [u32; 4],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VertexInput0 {
    pub position0: [f32; 4],
    pub normal0: [f32; 4],
    pub tangent0: [f32; 4],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VertexInput1 {
    pub map1_uvset: [f32; 4],
    pub uv_set1_uv_set2: [f32; 4],
    pub bake1: [f32; 4],
    pub color_set1: [f32; 4],
    pub color_set2_combined: [f32; 4],
    pub color_set3: [f32; 4],
    pub color_set4: [f32; 4],
    pub color_set5: [f32; 4],
    pub color_set6: [f32; 4],
    pub color_set7: [f32; 4],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VertexOutput {
    pub clip_position: [f32; 4],
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tangent: [f32; 4],
    pub map1_uvset: [f32; 4],
    pub uv_set1_uv_set2: [f32; 4],
    pub bake1: [f32; 2],
    pub color_set1: [f32; 4],
    pub color_set2_combined: [f32; 4],
    pub color_set3: [f32; 4],
    pub color_set4: [f32; 4],
    pub color_set5: [f32; 4],
    pub color_set6: [f32; 4],
    pub color_set7: [f32; 4],
    pub light_position: [f32; 4],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VertexOutputInvalid {
    pub clip_position: [f32; 4],
    pub position: [f32; 4],
}
pub mod bind_groups {
    pub struct BindGroup0(wgpu::BindGroup);
    pub struct BindGroupLayout0<'a> {
        pub camera: wgpu::BufferBinding<'a>,
        pub texture_shadow: &'a wgpu::TextureView,
        pub sampler_shadow: &'a wgpu::Sampler,
        pub light: wgpu::BufferBinding<'a>,
        pub render_settings: wgpu::BufferBinding<'a>,
        pub stage_uniforms: wgpu::BufferBinding<'a>,
    }
    const LAYOUT_DESCRIPTOR0: wgpu::BindGroupLayoutDescriptor = wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float {
                        filterable: true,
                    },
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 3,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 4,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 5,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    };
    impl BindGroup0 {
        pub fn get_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
            device.create_bind_group_layout(&LAYOUT_DESCRIPTOR0)
        }
        pub fn from_bindings(device: &wgpu::Device, bindings: BindGroupLayout0) -> Self {
            let bind_group_layout = device.create_bind_group_layout(&LAYOUT_DESCRIPTOR0);
            let bind_group = device
                .create_bind_group(
                    &wgpu::BindGroupDescriptor {
                        layout: &bind_group_layout,
                        entries: &[
                            wgpu::BindGroupEntry {
                                binding: 0,
                                resource: wgpu::BindingResource::Buffer(bindings.camera),
                            },
                            wgpu::BindGroupEntry {
                                binding: 1,
                                resource: wgpu::BindingResource::TextureView(
                                    bindings.texture_shadow,
                                ),
                            },
                            wgpu::BindGroupEntry {
                                binding: 2,
                                resource: wgpu::BindingResource::Sampler(
                                    bindings.sampler_shadow,
                                ),
                            },
                            wgpu::BindGroupEntry {
                                binding: 3,
                                resource: wgpu::BindingResource::Buffer(bindings.light),
                            },
                            wgpu::BindGroupEntry {
                                binding: 4,
                                resource: wgpu::BindingResource::Buffer(
                                    bindings.render_settings,
                                ),
                            },
                            wgpu::BindGroupEntry {
                                binding: 5,
                                resource: wgpu::BindingResource::Buffer(
                                    bindings.stage_uniforms,
                                ),
                            },
                        ],
                        label: None,
                    },
                );
            Self(bind_group)
        }
        pub fn set<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
            render_pass.set_bind_group(0, &self.0, &[]);
        }
    }
    pub struct BindGroup1(wgpu::BindGroup);
    pub struct BindGroupLayout1<'a> {
        pub texture0: &'a wgpu::TextureView,
        pub sampler0: &'a wgpu::Sampler,
        pub texture1: &'a wgpu::TextureView,
        pub sampler1: &'a wgpu::Sampler,
        pub texture2: &'a wgpu::TextureView,
        pub sampler2: &'a wgpu::Sampler,
        pub texture3: &'a wgpu::TextureView,
        pub sampler3: &'a wgpu::Sampler,
        pub texture4: &'a wgpu::TextureView,
        pub sampler4: &'a wgpu::Sampler,
        pub texture5: &'a wgpu::TextureView,
        pub sampler5: &'a wgpu::Sampler,
        pub texture6: &'a wgpu::TextureView,
        pub sampler6: &'a wgpu::Sampler,
        pub texture7: &'a wgpu::TextureView,
        pub sampler7: &'a wgpu::Sampler,
        pub texture8: &'a wgpu::TextureView,
        pub sampler8: &'a wgpu::Sampler,
        pub texture9: &'a wgpu::TextureView,
        pub sampler9: &'a wgpu::Sampler,
        pub texture10: &'a wgpu::TextureView,
        pub sampler10: &'a wgpu::Sampler,
        pub texture11: &'a wgpu::TextureView,
        pub sampler11: &'a wgpu::Sampler,
        pub texture12: &'a wgpu::TextureView,
        pub sampler12: &'a wgpu::Sampler,
        pub texture13: &'a wgpu::TextureView,
        pub sampler13: &'a wgpu::Sampler,
        pub texture14: &'a wgpu::TextureView,
        pub sampler14: &'a wgpu::Sampler,
        pub uniforms: wgpu::BufferBinding<'a>,
    }
    const LAYOUT_DESCRIPTOR1: wgpu::BindGroupLayoutDescriptor = wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float {
                        filterable: true,
                    },
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float {
                        filterable: true,
                    },
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 3,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 4,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::Cube,
                    sample_type: wgpu::TextureSampleType::Float {
                        filterable: true,
                    },
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 5,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 6,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float {
                        filterable: true,
                    },
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 7,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 8,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float {
                        filterable: true,
                    },
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 9,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 10,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float {
                        filterable: true,
                    },
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 11,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 12,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float {
                        filterable: true,
                    },
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 13,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 14,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::Cube,
                    sample_type: wgpu::TextureSampleType::Float {
                        filterable: true,
                    },
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 15,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 16,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::Cube,
                    sample_type: wgpu::TextureSampleType::Float {
                        filterable: true,
                    },
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 17,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 18,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float {
                        filterable: true,
                    },
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 19,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 20,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float {
                        filterable: true,
                    },
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 21,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 22,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float {
                        filterable: true,
                    },
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 23,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 24,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float {
                        filterable: true,
                    },
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 25,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 26,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float {
                        filterable: true,
                    },
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 27,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 28,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float {
                        filterable: true,
                    },
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 29,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 30,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    };
    impl BindGroup1 {
        pub fn get_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
            device.create_bind_group_layout(&LAYOUT_DESCRIPTOR1)
        }
        pub fn from_bindings(device: &wgpu::Device, bindings: BindGroupLayout1) -> Self {
            let bind_group_layout = device.create_bind_group_layout(&LAYOUT_DESCRIPTOR1);
            let bind_group = device
                .create_bind_group(
                    &wgpu::BindGroupDescriptor {
                        layout: &bind_group_layout,
                        entries: &[
                            wgpu::BindGroupEntry {
                                binding: 0,
                                resource: wgpu::BindingResource::TextureView(
                                    bindings.texture0,
                                ),
                            },
                            wgpu::BindGroupEntry {
                                binding: 1,
                                resource: wgpu::BindingResource::Sampler(bindings.sampler0),
                            },
                            wgpu::BindGroupEntry {
                                binding: 2,
                                resource: wgpu::BindingResource::TextureView(
                                    bindings.texture1,
                                ),
                            },
                            wgpu::BindGroupEntry {
                                binding: 3,
                                resource: wgpu::BindingResource::Sampler(bindings.sampler1),
                            },
                            wgpu::BindGroupEntry {
                                binding: 4,
                                resource: wgpu::BindingResource::TextureView(
                                    bindings.texture2,
                                ),
                            },
                            wgpu::BindGroupEntry {
                                binding: 5,
                                resource: wgpu::BindingResource::Sampler(bindings.sampler2),
                            },
                            wgpu::BindGroupEntry {
                                binding: 6,
                                resource: wgpu::BindingResource::TextureView(
                                    bindings.texture3,
                                ),
                            },
                            wgpu::BindGroupEntry {
                                binding: 7,
                                resource: wgpu::BindingResource::Sampler(bindings.sampler3),
                            },
                            wgpu::BindGroupEntry {
                                binding: 8,
                                resource: wgpu::BindingResource::TextureView(
                                    bindings.texture4,
                                ),
                            },
                            wgpu::BindGroupEntry {
                                binding: 9,
                                resource: wgpu::BindingResource::Sampler(bindings.sampler4),
                            },
                            wgpu::BindGroupEntry {
                                binding: 10,
                                resource: wgpu::BindingResource::TextureView(
                                    bindings.texture5,
                                ),
                            },
                            wgpu::BindGroupEntry {
                                binding: 11,
                                resource: wgpu::BindingResource::Sampler(bindings.sampler5),
                            },
                            wgpu::BindGroupEntry {
                                binding: 12,
                                resource: wgpu::BindingResource::TextureView(
                                    bindings.texture6,
                                ),
                            },
                            wgpu::BindGroupEntry {
                                binding: 13,
                                resource: wgpu::BindingResource::Sampler(bindings.sampler6),
                            },
                            wgpu::BindGroupEntry {
                                binding: 14,
                                resource: wgpu::BindingResource::TextureView(
                                    bindings.texture7,
                                ),
                            },
                            wgpu::BindGroupEntry {
                                binding: 15,
                                resource: wgpu::BindingResource::Sampler(bindings.sampler7),
                            },
                            wgpu::BindGroupEntry {
                                binding: 16,
                                resource: wgpu::BindingResource::TextureView(
                                    bindings.texture8,
                                ),
                            },
                            wgpu::BindGroupEntry {
                                binding: 17,
                                resource: wgpu::BindingResource::Sampler(bindings.sampler8),
                            },
                            wgpu::BindGroupEntry {
                                binding: 18,
                                resource: wgpu::BindingResource::TextureView(
                                    bindings.texture9,
                                ),
                            },
                            wgpu::BindGroupEntry {
                                binding: 19,
                                resource: wgpu::BindingResource::Sampler(bindings.sampler9),
                            },
                            wgpu::BindGroupEntry {
                                binding: 20,
                                resource: wgpu::BindingResource::TextureView(
                                    bindings.texture10,
                                ),
                            },
                            wgpu::BindGroupEntry {
                                binding: 21,
                                resource: wgpu::BindingResource::Sampler(bindings.sampler10),
                            },
                            wgpu::BindGroupEntry {
                                binding: 22,
                                resource: wgpu::BindingResource::TextureView(
                                    bindings.texture11,
                                ),
                            },
                            wgpu::BindGroupEntry {
                                binding: 23,
                                resource: wgpu::BindingResource::Sampler(bindings.sampler11),
                            },
                            wgpu::BindGroupEntry {
                                binding: 24,
                                resource: wgpu::BindingResource::TextureView(
                                    bindings.texture12,
                                ),
                            },
                            wgpu::BindGroupEntry {
                                binding: 25,
                                resource: wgpu::BindingResource::Sampler(bindings.sampler12),
                            },
                            wgpu::BindGroupEntry {
                                binding: 26,
                                resource: wgpu::BindingResource::TextureView(
                                    bindings.texture13,
                                ),
                            },
                            wgpu::BindGroupEntry {
                                binding: 27,
                                resource: wgpu::BindingResource::Sampler(bindings.sampler13),
                            },
                            wgpu::BindGroupEntry {
                                binding: 28,
                                resource: wgpu::BindingResource::TextureView(
                                    bindings.texture14,
                                ),
                            },
                            wgpu::BindGroupEntry {
                                binding: 29,
                                resource: wgpu::BindingResource::Sampler(bindings.sampler14),
                            },
                            wgpu::BindGroupEntry {
                                binding: 30,
                                resource: wgpu::BindingResource::Buffer(bindings.uniforms),
                            },
                        ],
                        label: None,
                    },
                );
            Self(bind_group)
        }
        pub fn set<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
            render_pass.set_bind_group(1, &self.0, &[]);
        }
    }
    pub struct BindGroups<'a> {
        pub bind_group0: &'a BindGroup0,
        pub bind_group1: &'a BindGroup1,
    }
    pub fn set_bind_groups<'a>(
        pass: &mut wgpu::RenderPass<'a>,
        bind_groups: BindGroups<'a>,
    ) {
        bind_groups.bind_group0.set(pass);
        bind_groups.bind_group1.set(pass);
    }
}
pub mod vertex {
    impl super::VertexInput0 {
        pub const VERTEX_ATTRIBUTES: [wgpu::VertexAttribute; 3] = wgpu::vertex_attr_array![
            0 => Float32x4, 1 => Float32x4, 2 => Float32x4
        ];
        /// The total size in bytes of all fields without considering padding or alignment.
        pub const SIZE_IN_BYTES: u64 = 48;
    }
    impl super::VertexInput1 {
        pub const VERTEX_ATTRIBUTES: [wgpu::VertexAttribute; 10] = wgpu::vertex_attr_array![
            3 => Float32x4, 4 => Float32x4, 5 => Float32x4, 6 => Float32x4, 7 =>
            Float32x4, 8 => Float32x4, 9 => Float32x4, 10 => Float32x4, 11 => Float32x4,
            12 => Float32x4
        ];
        /// The total size in bytes of all fields without considering padding or alignment.
        pub const SIZE_IN_BYTES: u64 = 160;
    }
}
pub fn create_shader_module(device: &wgpu::Device) -> wgpu::ShaderModule {
    let source = std::borrow::Cow::Borrowed(include_str!("model.wgsl"));
    device
        .create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(source),
        })
}
pub fn create_pipeline_layout(device: &wgpu::Device) -> wgpu::PipelineLayout {
    device
        .create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[
                    &bind_groups::BindGroup0::get_bind_group_layout(device),
                    &bind_groups::BindGroup1::get_bind_group_layout(device),
                ],
                push_constant_ranges: &[],
            },
        )
}
