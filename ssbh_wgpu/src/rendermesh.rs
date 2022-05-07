use crate::{
    animation::{animate_materials, animate_skel, animate_visibility, AnimationTransforms},
    pipeline::create_pipeline,
    texture::{load_sampler, load_texture},
    uniforms::{create_uniforms, create_uniforms_buffer},
    vertex::{mesh_object_buffers, MeshObjectBufferData},
    ModelFolder, PipelineData,
};
use ssbh_data::{
    adj_data::AdjEntryData,
    matl_data::{MatlEntryData, ParamId},
    mesh_data::MeshObjectData,
    prelude::*,
};
use std::{collections::HashMap, sync::Arc};
use wgpu::{util::DeviceExt, SamplerDescriptor, TextureViewDescriptor};

// Group resources shared between mesh objects.
// Shared resources can be updated once per model instead of per mesh.
// Keep most fields private since the buffer layout is an implementation detail.
// Assume render data is only shared within a folder.
// TODO: Associate animation folders with model folders?
// TODO: Is it worth allowing models to reference textures from other folders?
pub struct RenderModel {
    pub meshes: Vec<RenderMesh>,
    skel: Option<SkelData>,
    matl: Option<MatlData>,
    mesh_buffers: MeshBuffers,
    material_data_by_label: HashMap<String, MaterialData>,
    textures: Vec<(String, wgpu::Texture)>, // (file name, texture)
}

// A RenderMesh is view over a portion of the RenderModel data.
// TODO: All the render data should be owned by the RenderModel.
// Each RenderMesh corresponds to the data for a single draw call.
pub struct RenderMesh {
    pub name: String,
    pub is_visible: bool,
    material_label: String,
    shader_tag: String,
    sub_index: u64,
    // TODO: It may be worth sharing buffers in the future.
    buffer_data: MeshObjectBufferData,
    sort_bias: i32,
    normals_bind_group: crate::shader::renormal::bind_groups::BindGroup0,
    skinning_bind_group: crate::shader::skinning::bind_groups::BindGroup0,
    skinning_transforms_bind_group: crate::shader::skinning::bind_groups::BindGroup1,
    mesh_object_info_bind_group: crate::shader::skinning::bind_groups::BindGroup2,
    // Use an Arc since material and pipeline data is often shared.
    pipeline: Arc<wgpu::RenderPipeline>,
}

impl RenderMesh {
    pub fn render_order(&self) -> isize {
        render_pass_index(&self.shader_tag) + self.sort_bias as isize
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct PipelineIdentifier {
    shader_label: String,
    // Depth state is set per mesh rather than per material.
    // This means we can't always have one pipeline per material.
    // In practice, there will usually be one pipeline per material.
    enable_depth_write: bool,
    enable_depth_test: bool,
}

struct MaterialData {
    material_uniforms_bind_group: crate::shader::model::bind_groups::BindGroup1,
    uniforms_buffer: wgpu::Buffer,
}

struct MeshBuffers {
    // TODO: Share vertex buffers?
    skinning_transforms: wgpu::Buffer,
    world_transforms: wgpu::Buffer,
}

impl RenderModel {
    /// Reassign the mesh materials based on `modl`.
    /// This does not create materials that do not already exist.
    pub fn reassign_materials(&mut self, modl: &ModlData) {
        // TODO: There should be a separate pipeline to use if the material assignment fails?
        // TODO: How does in game handle invalid material labels?
        for mesh in &mut self.meshes {
            if let Some(entry) = modl.entries.iter().find(|e| {
                e.mesh_object_name == mesh.name && e.mesh_object_sub_index == mesh.sub_index
            }) {
                mesh.material_label = entry.material_label.clone();
            }
        }
    }

    /// Update the render data associated with `material`.
    pub fn update_material(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        material: &MatlEntryData,
        default_textures: &[(String, wgpu::Texture)],
        stage_cube: &(wgpu::TextureView, wgpu::Sampler),
    ) {
        if let Some(data) = self
            .material_data_by_label
            .get_mut(&material.material_label)
        {
            // TODO: Update textures and materials separately?
            let uniforms_buffer = create_uniforms_buffer(Some(material), device);

            data.material_uniforms_bind_group = create_material_uniforms_bind_group(
                Some(material),
                device,
                &self.textures,
                default_textures,
                stage_cube,
                &uniforms_buffer,
            );
            // let uniforms = create_uniforms(Some(material));

            // queue.write_buffer(&data.uniforms_buffer, 0, bytemuck::cast_slice(&[uniforms]));
        }
    }

    // TODO: Does it make sense to just pass None to "reset" the animation?
    pub fn apply_anim(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        anim: Option<&AnimData>,
        frame: f32,
        default_textures: &[(String, wgpu::Texture)],
        stage_cube: &(wgpu::TextureView, wgpu::Sampler),
    ) {
        // Update the buffers associated with each skel.
        // This avoids updating per mesh object and allocating new buffers.
        // TODO: How to "reset" an animation?

        if let Some(anim) = anim {
            animate_visibility(anim, frame, &mut self.meshes);

            if let Some(matl) = &self.matl {
                // Get a list of changed materials.
                let animated_materials = animate_materials(anim, frame, &matl.entries);
                for material in animated_materials {
                    // TODO: Should this go in a separate module?
                    // Get updated uniform buffers for animated materials
                    self.update_material(device, queue, &material, default_textures, stage_cube);
                }
            }

            if let Some(skel) = &self.skel {
                let animation_transforms = animate_skel(skel, anim, frame);
                queue.write_buffer(
                    &self.mesh_buffers.skinning_transforms,
                    0,
                    bytemuck::cast_slice(&[*animation_transforms.animated_world_transforms]),
                );

                queue.write_buffer(
                    &self.mesh_buffers.world_transforms,
                    0,
                    bytemuck::cast_slice(&(*animation_transforms.world_transforms)),
                );
            }
        }
    }

    pub fn draw_render_meshes<'a>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>,
        camera_bind_group: &'a crate::shader::model::bind_groups::BindGroup0,
        stage_uniforms_bind_group: &'a crate::shader::model::bind_groups::BindGroup2,
        shadow_bind_group: &'a crate::shader::model::bind_groups::BindGroup3,
    ) {
        for mesh in self.meshes.iter().filter(|m| m.is_visible) {
            render_pass.set_pipeline(mesh.pipeline.as_ref());

            // TODO: How to store all data in RenderModel but still draw sorted meshes?
            // TODO: Don't assume materials are properly assigned.
            let material_data = &self.material_data_by_label[&mesh.material_label];
            crate::shader::model::bind_groups::set_bind_groups(
                render_pass,
                crate::shader::model::bind_groups::BindGroups::<'a> {
                    bind_group0: camera_bind_group,
                    bind_group1: &material_data.material_uniforms_bind_group,
                    bind_group2: stage_uniforms_bind_group,
                    bind_group3: shadow_bind_group,
                },
            );

            mesh.set_vertex_buffers(render_pass);
            mesh.set_index_buffer(render_pass);

            render_pass.draw_indexed(0..mesh.buffer_data.vertex_index_count as u32, 0, 0..1);
        }
    }

    pub fn draw_render_meshes_depth<'a>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>,
        camera_bind_group: &'a crate::shader::model_depth::bind_groups::BindGroup0,
    ) {
        for mesh in self.meshes.iter().filter(|m| m.is_visible) {
            crate::shader::model_depth::bind_groups::set_bind_groups(
                render_pass,
                crate::shader::model_depth::bind_groups::BindGroups::<'a> {
                    bind_group0: camera_bind_group,
                },
            );

            mesh.set_vertex_buffers(render_pass);
            mesh.set_index_buffer(render_pass);

            render_pass.draw_indexed(0..mesh.buffer_data.vertex_index_count as u32, 0, 0..1);
        }
    }
}

impl RenderMesh {
    pub fn set_vertex_buffers<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        // TODO: Store the start/end indices in a tuple to avoid having to clone the range?
        render_pass.set_vertex_buffer(0, self.buffer_data.vertex_buffer0.slice(..));
        render_pass.set_vertex_buffer(1, self.buffer_data.vertex_buffer1.slice(..));
    }

    pub fn set_index_buffer<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        // TODO: Store the buffer and type together?
        render_pass.set_index_buffer(
            self.buffer_data.index_buffer.slice(..),
            wgpu::IndexFormat::Uint32,
        );
    }
}

// TODO: Come up with a more descriptive name for this.
pub struct RenderMeshSharedData<'a> {
    pub pipeline_data: &'a PipelineData,
    pub model: &'a ModelFolder,
    pub default_textures: &'a [(String, wgpu::Texture)],
    pub stage_cube: &'a (wgpu::TextureView, wgpu::Sampler),
}

pub fn create_render_model(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    shared_data: &RenderMeshSharedData,
) -> RenderModel {
    let start = std::time::Instant::now();

    // Attempt to initialize transforms using the skel.
    // This correctly positions mesh objects parented to a bone.
    // Otherwise, don't apply any transformations.
    // TODO: Is it worth matching the in game behavior for a missing skel?
    // "Invisible" models might be more confusing for users to understand.
    let anim_transforms = shared_data
        .model
        .skel
        .as_ref()
        .map(AnimationTransforms::from_skel)
        .unwrap_or_else(AnimationTransforms::identity);

    // Share the transforms buffer to avoid redundant updates.
    // TODO: Enforce bone count being at most 511?
    let skinning_transforms_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Bone Transforms Buffer"),
        contents: bytemuck::cast_slice(&[*anim_transforms.animated_world_transforms]),
        // COPY_DST allows applying animations without allocating new buffers
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let world_transforms_buffer =
        create_world_transforms_buffer(device, &anim_transforms.world_transforms);

    let mesh_buffers = MeshBuffers {
        skinning_transforms: skinning_transforms_buffer,
        world_transforms: world_transforms_buffer,
    };

    let (meshes, material_data_by_label, textures) =
        create_render_meshes(device, queue, &mesh_buffers, shared_data);

    println!(
        "Create {:?} render meshes and {:?} materials: {:?}",
        meshes.len(),
        material_data_by_label.len(),
        start.elapsed()
    );

    RenderModel {
        meshes,
        skel: shared_data.model.skel.clone(),
        matl: shared_data.model.matl.clone(),
        mesh_buffers,
        material_data_by_label,
        textures,
    }
}

fn create_material_data(
    device: &wgpu::Device,
    material: Option<&MatlEntryData>,
    textures: &[(String, wgpu::Texture)], // TODO: document that this uses file name?
    default_textures: &[(String, wgpu::Texture)], // TODO: document that this is an absolute path?
    stage_cube: &(wgpu::TextureView, wgpu::Sampler),
) -> MaterialData {
    let uniforms_buffer = create_uniforms_buffer(material, device);
    let material_uniforms_bind_group = create_material_uniforms_bind_group(
        material,
        device,
        textures,
        default_textures,
        stage_cube,
        &uniforms_buffer,
    );

    MaterialData {
        material_uniforms_bind_group,
        uniforms_buffer,
    }
}

fn create_render_meshes(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    mesh_buffers: &MeshBuffers,
    shared_data: &RenderMeshSharedData,
) -> (
    Vec<RenderMesh>,
    HashMap<String, MaterialData>,
    Vec<(String, wgpu::Texture)>,
) {
    // TODO: Find a way to organize this.

    // Initialize textures exactly once for performance.
    // Unused textures are rare, so we won't lazy load them.
    let textures: Vec<_> = shared_data
        .model
        .textures_by_file_name
        .iter()
        .map(|(name, nutexb)| {
            (
                name.clone(),
                nutexb_wgpu::create_texture(nutexb, device, queue),
            )
        })
        .collect();

    // Mesh objects control the depth state of the pipeline.
    // In practice, each (shader,mesh) pair may need a unique pipeline.
    // Cache materials separately since materials may share a pipeline.
    // TODO: How to test these optimizations?
    let mut pipelines = HashMap::new();

    // Similarly, materials can be shared between mesh objects.
    // All the pipelines use the same shader code,
    // so any MaterialData can be used with any pipeline.
    // TODO: Should red/yellow checkerboard errors just be separate pipelines?
    // It doesn't make sense to complicate the shader any further.
    // TODO: Split into PerMaterial, PerObject, etc in the shaders?
    // TODO: Handle missing materials?
    // TODO: Create a single "missing" material for meshes to use as a fallback?
    let material_data_by_label: HashMap<_, _> = shared_data
        .model
        .matl
        .as_ref()
        .unwrap()
        .entries
        .iter()
        .map(|entry| {
            let data = create_material_data(
                device,
                Some(entry),
                &textures,
                shared_data.default_textures,
                shared_data.stage_cube,
            );
            (entry.material_label.clone(), data)
        })
        .collect();

    // TODO: Share vertex buffers?
    // TODO: Find a way to have fewer function parameters?
    let meshes: Vec<_> = shared_data
        .model
        .mesh
        .objects
        .iter() // TODO: par_iter?
        .enumerate()
        .map(|(i, mesh_object)| {
            // Some mesh objects have associated triangle adjacency.
            let adj_entry = shared_data
                .model
                .adj
                .as_ref()
                .and_then(|adj| adj.entries.iter().find(|e| e.mesh_object_index == i));

            create_render_mesh(
                device,
                mesh_object,
                adj_entry,
                &mut pipelines,
                mesh_buffers,
                &textures,
                shared_data,
            )
        })
        .collect();

    (meshes, material_data_by_label, textures)
}

// TODO: Group these parameters?
fn create_render_mesh(
    device: &wgpu::Device,
    mesh_object: &MeshObjectData,
    adj_entry: Option<&AdjEntryData>,
    pipelines: &mut HashMap<PipelineIdentifier, Arc<wgpu::RenderPipeline>>,
    mesh_buffers: &MeshBuffers,
    textures: &[(String, wgpu::Texture)],
    shared_data: &RenderMeshSharedData,
) -> RenderMesh {
    // TODO: These could be cleaner as functions.
    // TODO: Is using a default for the material label ok?
    // TODO: How does a missing material work in game for missing matl/modl entry?
    let material_label = shared_data
        .model
        .modl
        .as_ref()
        .and_then(|m| {
            m.entries
                .iter()
                .find(|e| {
                    e.mesh_object_name == mesh_object.name
                        && e.mesh_object_sub_index == mesh_object.sub_index
                })
                .map(|e| &e.material_label)
        })
        .unwrap_or(&String::new())
        .to_string();

    let material = shared_data.model.matl.as_ref().and_then(|matl| {
        matl.entries
            .iter()
            .find(|e| e.material_label == material_label)
    });

    // Pipeline creation is expensive.
    // Lazily initialize pipelines and share pipelines when possible.
    let pipeline = pipelines
        .entry(PipelineIdentifier {
            // Strip the shader tag since it doesn't effect the pipeline itself.
            // TODO: Is this always a safe assumption?
            shader_label: material
                .and_then(|material| material.shader_label.get(0..24))
                .unwrap_or_default()
                .to_string(),
            enable_depth_write: !mesh_object.disable_depth_write,
            enable_depth_test: !mesh_object.disable_depth_test,
        })
        .or_insert_with(|| {
            Arc::new(create_pipeline(
                device,
                shared_data.pipeline_data,
                material,
                !mesh_object.disable_depth_write,
                !mesh_object.disable_depth_test,
            ))
        });

    let buffer_data = mesh_object_buffers(device, mesh_object, shared_data.model.skel.as_ref());

    // TODO: Function for this?
    let adjacency = adj_entry
        .map(|e| e.vertex_adjacency.iter().map(|i| *i as i32).collect())
        .unwrap_or_else(|| vec![-1i32; mesh_object.vertex_count().unwrap() * 18]);
    let adj_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("vertex buffer 0"),
        contents: bytemuck::cast_slice(&adjacency),
        usage: wgpu::BufferUsages::STORAGE,
    });

    // This is applied after skinning, so the source and destination buffer are the same.
    let renormal_bind_group = crate::shader::renormal::bind_groups::BindGroup0::from_bindings(
        device,
        crate::shader::renormal::bind_groups::BindGroupLayout0 {
            vertices: &buffer_data.vertex_buffer0,
            adj_data: &adj_buffer,
        },
    );

    let skinning_bind_group = crate::shader::skinning::bind_groups::BindGroup0::from_bindings(
        device,
        crate::shader::skinning::bind_groups::BindGroupLayout0 {
            src: &buffer_data.vertex_buffer0_source,
            vertex_weights: &buffer_data.skinning_buffer,
            dst: &buffer_data.vertex_buffer0,
        },
    );

    let skinning_transforms_bind_group =
        crate::shader::skinning::bind_groups::BindGroup1::from_bindings(
            device,
            crate::shader::skinning::bind_groups::BindGroupLayout1 {
                transforms: &mesh_buffers.skinning_transforms,
                world_transforms: &mesh_buffers.world_transforms,
            },
        );

    let parent_index = find_parent_index(mesh_object, &shared_data.model.skel);
    let mesh_object_info_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Mesh Object Info Buffer"),
        contents: bytemuck::cast_slice(&[crate::shader::skinning::MeshObjectInfo {
            parent_index: [parent_index, -1, -1, -1],
        }]),
        usage: wgpu::BufferUsages::UNIFORM,
    });

    let mesh_object_info_bind_group =
        crate::shader::skinning::bind_groups::BindGroup2::from_bindings(
            device,
            crate::shader::skinning::bind_groups::BindGroupLayout2 {
                mesh_object_info: &mesh_object_info_buffer,
            },
        );

    // The end of the shader label is used to determine draw order.
    // ex: "SFX_PBS_0101000008018278_sort" has a tag of "sort".
    // The render order is opaque -> far -> sort -> near.
    // TODO: How to handle missing tags?
    let shader_tag = material
        .and_then(|m| m.shader_label.get(25..))
        .unwrap_or("")
        .to_string();
    RenderMesh {
        name: mesh_object.name.clone(),
        material_label: material_label.clone(),
        shader_tag,
        is_visible: true,
        buffer_data,
        sort_bias: mesh_object.sort_bias,
        skinning_bind_group,
        skinning_transforms_bind_group,
        mesh_object_info_bind_group,
        pipeline: pipeline.clone(),
        normals_bind_group: renormal_bind_group,
        sub_index: mesh_object.sub_index,
    }
}

fn create_material_uniforms_bind_group(
    material: Option<&ssbh_data::matl_data::MatlEntryData>,
    device: &wgpu::Device,
    textures: &[(String, wgpu::Texture)],
    default_textures: &[(String, wgpu::Texture)],
    stage_cube: &(wgpu::TextureView, wgpu::Sampler),
    uniforms_buffer: &wgpu::Buffer, // TODO: Just return this?
) -> crate::shader::model::bind_groups::BindGroup1 {
    // TODO: Do all textures default to white if the path isn't correct?
    // TODO: Default cube map?
    let default_white = &default_textures
        .iter()
        .find(|d| d.0 == "/common/shader/sfxpbs/default_white")
        .unwrap()
        .1;

    let load_texture = |texture_id| {
        load_texture(material, texture_id, textures, default_textures)
            .unwrap_or_else(|| default_white.create_view(&TextureViewDescriptor::default()))
    };

    let load_sampler = |sampler_id| {
        load_sampler(material, device, sampler_id)
            .unwrap_or_else(|| device.create_sampler(&SamplerDescriptor::default()))
    };

    // TODO: Better cube map handling.
    // TODO: Default texture for other cube maps?

    // TODO: How to enforce certain textures being cube maps?
    crate::shader::model::bind_groups::BindGroup1::from_bindings(
        device,
        crate::shader::model::bind_groups::BindGroupLayout1 {
            texture0: &load_texture(ParamId::Texture0),
            sampler0: &load_sampler(ParamId::Sampler0),
            texture1: &load_texture(ParamId::Texture1),
            sampler1: &load_sampler(ParamId::Sampler1),
            texture2: &stage_cube.0,
            sampler2: &load_sampler(ParamId::Sampler2),
            texture3: &load_texture(ParamId::Texture3),
            sampler3: &load_sampler(ParamId::Sampler3),
            texture4: &load_texture(ParamId::Texture4),
            sampler4: &load_sampler(ParamId::Sampler4),
            texture5: &load_texture(ParamId::Texture5),
            sampler5: &load_sampler(ParamId::Sampler5),
            texture6: &load_texture(ParamId::Texture6),
            sampler6: &load_sampler(ParamId::Sampler6),
            texture7: &stage_cube.0,
            sampler7: &load_sampler(ParamId::Sampler7),
            texture8: &stage_cube.0,
            sampler8: &load_sampler(ParamId::Sampler8),
            texture9: &load_texture(ParamId::Texture9),
            sampler9: &load_sampler(ParamId::Sampler9),
            texture10: &load_texture(ParamId::Texture10),
            sampler10: &load_sampler(ParamId::Sampler10),
            texture11: &load_texture(ParamId::Texture11),
            sampler11: &load_sampler(ParamId::Sampler11),
            texture12: &load_texture(ParamId::Texture12),
            sampler12: &load_sampler(ParamId::Sampler12),
            texture13: &load_texture(ParamId::Texture13),
            sampler13: &load_sampler(ParamId::Sampler13),
            texture14: &load_texture(ParamId::Texture14),
            sampler14: &load_sampler(ParamId::Sampler14),
            uniforms: uniforms_buffer,
        },
    )
}

// TODO: Where to put this?
// TODO: Module for skinning buffers?
fn create_world_transforms_buffer(
    device: &wgpu::Device,
    animated_world_transforms: &[glam::Mat4; 512],
) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("World Transforms Buffer"),
        contents: bytemuck::cast_slice(animated_world_transforms),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    })
}

fn find_parent_index(mesh_object: &MeshObjectData, skel: &Option<SkelData>) -> i32 {
    // Only include a parent if there are no bone influences.
    // TODO: What happens if there are influences and a parent bone?
    if mesh_object.bone_influences.is_empty() {
        skel.as_ref()
            .and_then(|skel| {
                skel.bones
                    .iter()
                    .position(|b| b.name == mesh_object.parent_bone_name)
            })
            .map(|i| i as i32)
            .unwrap_or(-1)
    } else {
        -1
    }
}

fn render_pass_index(tag: &str) -> isize {
    match tag {
        "opaque" => 0,
        "far" => 1,
        "sort" => 2,
        "near" => 3,
        _ => 0, // TODO: How to handle invalid tags?
    }
}

pub fn dispatch_renormal<'a>(meshes: &'a [RenderMesh], compute_pass: &mut wgpu::ComputePass<'a>) {
    // Assume the pipeline is already set.
    // Some meshes have a material label tag to enable the recalculating of normals.
    // This helps with animations with large deformations.
    // TODO: Is this check case sensitive?
    for mesh in meshes
        .iter()
        .filter(|m| m.material_label.contains("RENORMAL"))
    {
        crate::shader::renormal::bind_groups::set_bind_groups(
            compute_pass,
            crate::shader::renormal::bind_groups::BindGroups::<'a> {
                bind_group0: &mesh.normals_bind_group,
            },
        );

        // The shader's local workgroup size is (256, 1, 1).
        // Round up to avoid skipping vertices.
        let workgroup_count = (mesh.buffer_data.vertex_count as f64 / 256.0).ceil() as u32;
        compute_pass.dispatch(workgroup_count, 1, 1);
    }
}

pub fn dispatch_skinning<'a>(meshes: &'a [RenderMesh], compute_pass: &mut wgpu::ComputePass<'a>) {
    // Assume the pipeline is already set.
    for mesh in meshes {
        crate::shader::skinning::bind_groups::set_bind_groups(
            compute_pass,
            crate::shader::skinning::bind_groups::BindGroups::<'a> {
                bind_group0: &mesh.skinning_bind_group,
                bind_group1: &mesh.skinning_transforms_bind_group,
                bind_group2: &mesh.mesh_object_info_bind_group,
            },
        );

        // The shader's local workgroup size is (256, 1, 1).
        // Round up to avoid skipping vertices.
        let workgroup_count = (mesh.buffer_data.vertex_count as f64 / 256.0).ceil() as u32;
        compute_pass.dispatch(workgroup_count, 1, 1);
    }
}
