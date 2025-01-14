// File automatically generated by build.rs.
// Changes made to this file will not be saved.
#[repr(C)]
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    bytemuck::Pod,
    bytemuck::Zeroable,
    encase::ShaderType
)]
pub struct VertexInput {
    pub position: glam::Vec4,
}
const _: () = assert!(
    std::mem::size_of:: < VertexInput > () == 16,
    "size of VertexInput does not match WGSL"
);
const _: () = assert!(
    memoffset::offset_of!(VertexInput, position) == 0,
    "offset of VertexInput.position does not match WGSL"
);
#[repr(C)]
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    bytemuck::Pod,
    bytemuck::Zeroable,
    encase::ShaderType
)]
pub struct CameraTransforms {
    pub model_view_matrix: glam::Mat4,
    pub mvp_matrix: glam::Mat4,
    pub mvp_inv_matrix: glam::Mat4,
    pub camera_pos: glam::Vec4,
    pub screen_dimensions: glam::Vec4,
}
const _: () = assert!(
    std::mem::size_of:: < CameraTransforms > () == 224,
    "size of CameraTransforms does not match WGSL"
);
const _: () = assert!(
    memoffset::offset_of!(CameraTransforms, model_view_matrix) == 0,
    "offset of CameraTransforms.model_view_matrix does not match WGSL"
);
const _: () = assert!(
    memoffset::offset_of!(CameraTransforms, mvp_matrix) == 64,
    "offset of CameraTransforms.mvp_matrix does not match WGSL"
);
const _: () = assert!(
    memoffset::offset_of!(CameraTransforms, mvp_inv_matrix) == 128,
    "offset of CameraTransforms.mvp_inv_matrix does not match WGSL"
);
const _: () = assert!(
    memoffset::offset_of!(CameraTransforms, camera_pos) == 192,
    "offset of CameraTransforms.camera_pos does not match WGSL"
);
const _: () = assert!(
    memoffset::offset_of!(CameraTransforms, screen_dimensions) == 208,
    "offset of CameraTransforms.screen_dimensions does not match WGSL"
);
pub mod bind_groups {
    pub struct BindGroup0(wgpu::BindGroup);
    pub struct BindGroupLayout0<'a> {
        pub camera: wgpu::BufferBinding<'a>,
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
    pub struct BindGroups<'a> {
        pub bind_group0: &'a BindGroup0,
    }
    pub fn set_bind_groups<'a>(
        pass: &mut wgpu::RenderPass<'a>,
        bind_groups: BindGroups<'a>,
    ) {
        bind_groups.bind_group0.set(pass);
    }
}
pub mod vertex {
    impl super::VertexInput {
        pub const VERTEX_ATTRIBUTES: [wgpu::VertexAttribute; 1] = [
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x4,
                offset: memoffset::offset_of!(super::VertexInput, position) as u64,
                shader_location: 0,
            },
        ];
        pub fn vertex_buffer_layout(
            step_mode: wgpu::VertexStepMode,
        ) -> wgpu::VertexBufferLayout<'static> {
            wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<super::VertexInput>() as u64,
                step_mode,
                attributes: &super::VertexInput::VERTEX_ATTRIBUTES,
            }
        }
    }
}
pub fn create_shader_module(device: &wgpu::Device) -> wgpu::ShaderModule {
    let source = std::borrow::Cow::Borrowed(include_str!("floor_grid.wgsl"));
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
                ],
                push_constant_ranges: &[],
            },
        )
}
