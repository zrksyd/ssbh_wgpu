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
pub struct VertexInput0 {
    pub position0: glam::Vec4,
    pub normal0: glam::Vec4,
    pub tangent0: glam::Vec4,
}
const _: () = assert!(
    std::mem::size_of:: < VertexInput0 > () == 48,
    "size of VertexInput0 does not match WGSL"
);
const _: () = assert!(
    memoffset::offset_of!(VertexInput0, position0) == 0,
    "offset of VertexInput0.position0 does not match WGSL"
);
const _: () = assert!(
    memoffset::offset_of!(VertexInput0, normal0) == 16,
    "offset of VertexInput0.normal0 does not match WGSL"
);
const _: () = assert!(
    memoffset::offset_of!(VertexInput0, tangent0) == 32,
    "offset of VertexInput0.tangent0 does not match WGSL"
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
pub struct VertexWeight {
    pub bone_indices: glam::IVec4,
    pub weights: glam::Vec4,
}
const _: () = assert!(
    std::mem::size_of:: < VertexWeight > () == 32,
    "size of VertexWeight does not match WGSL"
);
const _: () = assert!(
    memoffset::offset_of!(VertexWeight, bone_indices) == 0,
    "offset of VertexWeight.bone_indices does not match WGSL"
);
const _: () = assert!(
    memoffset::offset_of!(VertexWeight, weights) == 16,
    "offset of VertexWeight.weights does not match WGSL"
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
pub struct AnimatedWorldTransforms {
    pub transforms: [glam::Mat4; 512],
    pub transforms_inv_transpose: [glam::Mat4; 512],
}
const _: () = assert!(
    std::mem::size_of:: < AnimatedWorldTransforms > () == 65536,
    "size of AnimatedWorldTransforms does not match WGSL"
);
const _: () = assert!(
    memoffset::offset_of!(AnimatedWorldTransforms, transforms) == 0,
    "offset of AnimatedWorldTransforms.transforms does not match WGSL"
);
const _: () = assert!(
    memoffset::offset_of!(AnimatedWorldTransforms, transforms_inv_transpose) == 32768,
    "offset of AnimatedWorldTransforms.transforms_inv_transpose does not match WGSL"
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
pub struct WorldTransforms {
    pub transforms: [glam::Mat4; 512],
}
const _: () = assert!(
    std::mem::size_of:: < WorldTransforms > () == 32768,
    "size of WorldTransforms does not match WGSL"
);
const _: () = assert!(
    memoffset::offset_of!(WorldTransforms, transforms) == 0,
    "offset of WorldTransforms.transforms does not match WGSL"
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
pub struct MeshObjectInfo {
    pub parent_index: glam::IVec4,
}
const _: () = assert!(
    std::mem::size_of:: < MeshObjectInfo > () == 16,
    "size of MeshObjectInfo does not match WGSL"
);
const _: () = assert!(
    memoffset::offset_of!(MeshObjectInfo, parent_index) == 0,
    "offset of MeshObjectInfo.parent_index does not match WGSL"
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
pub struct SkinningSettings {
    pub enable_parenting: glam::UVec4,
    pub enable_skinning: glam::UVec4,
}
const _: () = assert!(
    std::mem::size_of:: < SkinningSettings > () == 32,
    "size of SkinningSettings does not match WGSL"
);
const _: () = assert!(
    memoffset::offset_of!(SkinningSettings, enable_parenting) == 0,
    "offset of SkinningSettings.enable_parenting does not match WGSL"
);
const _: () = assert!(
    memoffset::offset_of!(SkinningSettings, enable_skinning) == 16,
    "offset of SkinningSettings.enable_skinning does not match WGSL"
);
pub mod bind_groups {
    pub struct BindGroup0(wgpu::BindGroup);
    pub struct BindGroupLayout0<'a> {
        pub src: wgpu::BufferBinding<'a>,
        pub vertex_weights: wgpu::BufferBinding<'a>,
        pub dst: wgpu::BufferBinding<'a>,
    }
    const LAYOUT_DESCRIPTOR0: wgpu::BindGroupLayoutDescriptor = wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage {
                        read_only: true,
                    },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage {
                        read_only: true,
                    },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage {
                        read_only: false,
                    },
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
                                resource: wgpu::BindingResource::Buffer(bindings.src),
                            },
                            wgpu::BindGroupEntry {
                                binding: 1,
                                resource: wgpu::BindingResource::Buffer(
                                    bindings.vertex_weights,
                                ),
                            },
                            wgpu::BindGroupEntry {
                                binding: 2,
                                resource: wgpu::BindingResource::Buffer(bindings.dst),
                            },
                        ],
                        label: None,
                    },
                );
            Self(bind_group)
        }
        pub fn set<'a>(&'a self, render_pass: &mut wgpu::ComputePass<'a>) {
            render_pass.set_bind_group(0, &self.0, &[]);
        }
    }
    pub struct BindGroup1(wgpu::BindGroup);
    pub struct BindGroupLayout1<'a> {
        pub transforms: wgpu::BufferBinding<'a>,
        pub world_transforms: wgpu::BufferBinding<'a>,
    }
    const LAYOUT_DESCRIPTOR1: wgpu::BindGroupLayoutDescriptor = wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::COMPUTE,
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
                                resource: wgpu::BindingResource::Buffer(bindings.transforms),
                            },
                            wgpu::BindGroupEntry {
                                binding: 1,
                                resource: wgpu::BindingResource::Buffer(
                                    bindings.world_transforms,
                                ),
                            },
                        ],
                        label: None,
                    },
                );
            Self(bind_group)
        }
        pub fn set<'a>(&'a self, render_pass: &mut wgpu::ComputePass<'a>) {
            render_pass.set_bind_group(1, &self.0, &[]);
        }
    }
    pub struct BindGroup2(wgpu::BindGroup);
    pub struct BindGroupLayout2<'a> {
        pub mesh_object_info: wgpu::BufferBinding<'a>,
    }
    const LAYOUT_DESCRIPTOR2: wgpu::BindGroupLayoutDescriptor = wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    };
    impl BindGroup2 {
        pub fn get_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
            device.create_bind_group_layout(&LAYOUT_DESCRIPTOR2)
        }
        pub fn from_bindings(device: &wgpu::Device, bindings: BindGroupLayout2) -> Self {
            let bind_group_layout = device.create_bind_group_layout(&LAYOUT_DESCRIPTOR2);
            let bind_group = device
                .create_bind_group(
                    &wgpu::BindGroupDescriptor {
                        layout: &bind_group_layout,
                        entries: &[
                            wgpu::BindGroupEntry {
                                binding: 0,
                                resource: wgpu::BindingResource::Buffer(
                                    bindings.mesh_object_info,
                                ),
                            },
                        ],
                        label: None,
                    },
                );
            Self(bind_group)
        }
        pub fn set<'a>(&'a self, render_pass: &mut wgpu::ComputePass<'a>) {
            render_pass.set_bind_group(2, &self.0, &[]);
        }
    }
    pub struct BindGroup3(wgpu::BindGroup);
    pub struct BindGroupLayout3<'a> {
        pub settings: wgpu::BufferBinding<'a>,
    }
    const LAYOUT_DESCRIPTOR3: wgpu::BindGroupLayoutDescriptor = wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    };
    impl BindGroup3 {
        pub fn get_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
            device.create_bind_group_layout(&LAYOUT_DESCRIPTOR3)
        }
        pub fn from_bindings(device: &wgpu::Device, bindings: BindGroupLayout3) -> Self {
            let bind_group_layout = device.create_bind_group_layout(&LAYOUT_DESCRIPTOR3);
            let bind_group = device
                .create_bind_group(
                    &wgpu::BindGroupDescriptor {
                        layout: &bind_group_layout,
                        entries: &[
                            wgpu::BindGroupEntry {
                                binding: 0,
                                resource: wgpu::BindingResource::Buffer(bindings.settings),
                            },
                        ],
                        label: None,
                    },
                );
            Self(bind_group)
        }
        pub fn set<'a>(&'a self, render_pass: &mut wgpu::ComputePass<'a>) {
            render_pass.set_bind_group(3, &self.0, &[]);
        }
    }
    pub struct BindGroups<'a> {
        pub bind_group0: &'a BindGroup0,
        pub bind_group1: &'a BindGroup1,
        pub bind_group2: &'a BindGroup2,
        pub bind_group3: &'a BindGroup3,
    }
    pub fn set_bind_groups<'a>(
        pass: &mut wgpu::ComputePass<'a>,
        bind_groups: BindGroups<'a>,
    ) {
        bind_groups.bind_group0.set(pass);
        bind_groups.bind_group1.set(pass);
        bind_groups.bind_group2.set(pass);
        bind_groups.bind_group3.set(pass);
    }
}
pub mod compute {
    pub const MAIN_WORKGROUP_SIZE: [u32; 3] = [256, 1, 1];
}
pub fn create_shader_module(device: &wgpu::Device) -> wgpu::ShaderModule {
    let source = std::borrow::Cow::Borrowed(include_str!("skinning.wgsl"));
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
                    &bind_groups::BindGroup2::get_bind_group_layout(device),
                    &bind_groups::BindGroup3::get_bind_group_layout(device),
                ],
                push_constant_ranges: &[],
            },
        )
}
