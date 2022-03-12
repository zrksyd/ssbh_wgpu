// File automatically generated by build.rs.
// Changes made to this file will not be saved.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VertexInput0 {
    pub position0: [f32; 4],
    pub normal0: [f32; 4],
    pub tangent0: [f32; 4],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VertexWeight {
    pub bone_indices: [i32; 4],
    pub weights: [f32; 4],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertices {
    pub vertices: [VertexInput0; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VertexWeights {
    pub vertices: [VertexWeight; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Transforms {
    pub transforms: [glam::Mat4; 512],
    pub transforms_inv_transpose: [glam::Mat4; 512],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct WorldTransforms {
    pub transforms: [glam::Mat4; 512],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MeshObjectInfo {
    pub parent_index: [i32; 4],
}
pub mod bind_groups {
    pub struct BindGroup0(wgpu::BindGroup);
    pub struct BindGroupLayout0<'a> {
        pub src: &'a wgpu::Buffer,
        pub vertex_weights: &'a wgpu::Buffer,
        pub dst: &'a wgpu::Buffer,
    }
    const LAYOUT_DESCRIPTOR0: wgpu::BindGroupLayoutDescriptor = wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0u32,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1u32,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2u32,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ]
    };
    impl BindGroup0 {
        pub fn get_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
            device.create_bind_group_layout(&LAYOUT_DESCRIPTOR0)
        }
    
        pub fn from_bindings(device: &wgpu::Device, bindings: BindGroupLayout0) -> Self {
            let bind_group_layout = device.create_bind_group_layout(&LAYOUT_DESCRIPTOR0);
            let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0u32,
                        resource: bindings.src.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1u32,
                        resource: bindings.vertex_weights.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2u32,
                        resource: bindings.dst.as_entire_binding(),
                    },
                ],
                label: None,
            });
            Self(bind_group)
        }
    
        pub fn set<'a>(&'a self, render_pass: &mut wgpu::ComputePass<'a>) {
            render_pass.set_bind_group(0u32, &self.0, &[]);
        }
    }
    pub struct BindGroup1(wgpu::BindGroup);
    pub struct BindGroupLayout1<'a> {
        pub transforms: &'a wgpu::Buffer,
        pub world_transforms: &'a wgpu::Buffer,
    }
    const LAYOUT_DESCRIPTOR1: wgpu::BindGroupLayoutDescriptor = wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0u32,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1u32,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ]
    };
    impl BindGroup1 {
        pub fn get_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
            device.create_bind_group_layout(&LAYOUT_DESCRIPTOR1)
        }
    
        pub fn from_bindings(device: &wgpu::Device, bindings: BindGroupLayout1) -> Self {
            let bind_group_layout = device.create_bind_group_layout(&LAYOUT_DESCRIPTOR1);
            let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0u32,
                        resource: bindings.transforms.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1u32,
                        resource: bindings.world_transforms.as_entire_binding(),
                    },
                ],
                label: None,
            });
            Self(bind_group)
        }
    
        pub fn set<'a>(&'a self, render_pass: &mut wgpu::ComputePass<'a>) {
            render_pass.set_bind_group(1u32, &self.0, &[]);
        }
    }
    pub struct BindGroup2(wgpu::BindGroup);
    pub struct BindGroupLayout2<'a> {
        pub mesh_object_info: &'a wgpu::Buffer,
    }
    const LAYOUT_DESCRIPTOR2: wgpu::BindGroupLayoutDescriptor = wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0u32,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ]
    };
    impl BindGroup2 {
        pub fn get_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
            device.create_bind_group_layout(&LAYOUT_DESCRIPTOR2)
        }
    
        pub fn from_bindings(device: &wgpu::Device, bindings: BindGroupLayout2) -> Self {
            let bind_group_layout = device.create_bind_group_layout(&LAYOUT_DESCRIPTOR2);
            let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0u32,
                        resource: bindings.mesh_object_info.as_entire_binding(),
                    },
                ],
                label: None,
            });
            Self(bind_group)
        }
    
        pub fn set<'a>(&'a self, render_pass: &mut wgpu::ComputePass<'a>) {
            render_pass.set_bind_group(2u32, &self.0, &[]);
        }
    }
    pub struct BindGroups<'a> {
        pub bind_group0: &'a BindGroup0,
        pub bind_group1: &'a BindGroup1,
        pub bind_group2: &'a BindGroup2,
    }
    pub fn set_bind_groups<'a>(
        pass: &mut wgpu::ComputePass<'a>,
        bind_groups: BindGroups<'a>,
    ) {
        pass.set_bind_group(0u32, &bind_groups.bind_group0.0, &[]);
        pass.set_bind_group(1u32, &bind_groups.bind_group1.0, &[]);
        pass.set_bind_group(2u32, &bind_groups.bind_group2.0, &[]);
    }
}
pub mod vertex {
}
pub fn create_shader_module(device: &wgpu::Device) -> wgpu::ShaderModule {
    device.create_shader_module(&wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!("skinning.wgsl")))
    })
}
pub fn create_pipeline_layout(device: &wgpu::Device) -> wgpu::PipelineLayout {
    device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[
            &bind_groups::BindGroup0::get_bind_group_layout(device),
            &bind_groups::BindGroup1::get_bind_group_layout(device),
            &bind_groups::BindGroup2::get_bind_group_layout(device),
        ],
        push_constant_ranges: &[],
    })
}