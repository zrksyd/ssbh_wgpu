use std::collections::HashSet;

use ssbh_data::{
    anim_data::{GroupType, TrackValues, Vector3, Vector4},
    prelude::*,
    skel_data::{BoneData, BoneTransformError},
};

use crate::{shader::skinning::Transforms, RenderMesh};

// Animation process is Skel, Anim -> Vec<AnimatedBone> -> [Mat4; 512], [Mat4; 512] -> Buffers?
// Associate an optional transform to override each bone?
// Evaluate the "tree" of Vec<AnimatedBone> to compute the final world transforms.
struct AnimatedBone {
    bone: BoneData,
    // TODO: Why does decomposing the bone transform itself not work?
    anim_transform: Option<AnimTransform>,
    compensate_scale: bool,
    inherit_scale: bool,
}

impl AnimatedBone {
    fn animated_transform(&self) -> glam::Mat4 {
        self.anim_transform
            .as_ref()
            .map(transform_to_mat4)
            .unwrap_or_else(|| mat4_from_row2d(&self.bone.transform))
    }
}

struct AnimTransform {
    translation: glam::Vec3,
    rotation: glam::Quat,
    scale: glam::Vec3,
}

pub struct AnimationTransforms {
    // Box large arrays to avoid stack overflows in debug mode.
    pub transforms: Box<Transforms>,
    pub world_transforms: Box<[glam::Mat4; 512]>,
}

// TODO: How to test this?
// TODO: Create test cases for test animations on short bone chains?
// TODO: Also apply visibility and material animation?
// TODO: Make this return the visibility info to make it easier to test?
pub fn apply_animation(
    skel: &SkelData,
    anim: Option<&AnimData>,
    frame: f32,
    meshes: &mut [RenderMesh],
) -> AnimationTransforms {
    // TODO: Make the skel optional?

    // TODO: Is this redundant?
    let mut anim_world_transforms = [glam::Mat4::IDENTITY; 512];
    let mut transforms = [glam::Mat4::IDENTITY; 512];

    // HACK: Duplicate the bone heirachy and override with the anim nodes.
    // A proper solution will take into account scaling, interpolation, etc.
    // This should be its own module or potentially a separate package.
    // TODO: There's probably an efficient execution order using the heirarchy.
    // There might not be enough bones to benefit from parallel execution.
    // We won't worry about redundant matrix multiplications for now.
    let mut animated_bones: Vec<_> = skel
        .bones
        .iter()
        .map(|b| AnimatedBone {
            bone: b.clone(),
            compensate_scale: false,
            inherit_scale: true,
            anim_transform: None,
        })
        .collect();

    if let Some(anim) = anim {
        animate_skel(anim, &mut animated_bones, frame, meshes);
    }

    // TODO: Create a second skel clone here?
    // Apply scale compensation now that parent values are known?

    // TODO: Reimplement calculate_world to take into account scale inheritance.
    // TODO: Should scale inheritance be part of ssbh_data itself?
    // TODO: Is there a more efficient way of calculating this?

    for (i, bone) in skel.bones.iter().enumerate() {
        // Smash is row-major but glam is column-major.
        // TODO: Is there an efficient way to calculate world transforms of all bones?
        let bone_world = skel.calculate_world_transform(bone).unwrap();
        let bone_world = glam::Mat4::from_cols_array_2d(&bone_world).transpose();

        // TODO: Apply animations?
        // TODO: Separate module with tests to check edge cases.

        let bone_anim_world = calculate_anim_world_transform(
            &animated_bones,
            animated_bones
                .iter()
                .find(|b| b.bone.name == bone.name)
                .unwrap(),
        )
        .unwrap();
        let bone_anim_world = glam::Mat4::from_cols_array_2d(&bone_anim_world).transpose();

        // TODO: Is wgpu expecting row-major?
        transforms[i] = (bone_world.inverse() * bone_anim_world).transpose();
        anim_world_transforms[i] = bone_anim_world.transpose();
    }

    let transforms_inv_transpose = transforms.map(|t| t.inverse().transpose());

    AnimationTransforms {
        world_transforms: Box::new(anim_world_transforms),
        transforms: Box::new(Transforms {
            transforms,
            transforms_inv_transpose,
        }),
    }
}

fn mat4_from_row2d(elements: &[[f32; 4]; 4]) -> glam::Mat4 {
    glam::Mat4::from_cols_array_2d(elements).transpose()
}

// TODO: Add scale inheritance here?
fn calculate_anim_world_transform(
    bones: &[AnimatedBone],
    bone: &AnimatedBone,
) -> Result<[[f32; 4]; 4], BoneTransformError> {
    let mut bone = bone;

    // TODO: Account for inheritance while recursing.
    let mut transform = bone.animated_transform();

    // Check for cycles by keeping track of previously visited locations.
    // TODO: Validate the skel once for cycles to make this step faster?
    let mut visited = HashSet::new();

    // Accumulate transforms by travelling up the bone heirarchy.
    while let Some(parent_index) = bone.bone.parent_index {
        if !visited.insert(parent_index) {
            return Err(BoneTransformError::CycleDetected {
                index: parent_index,
            });
        }
        if let Some(parent_bone) = bones.get(parent_index) {
            let parent_transform = parent_bone.animated_transform();

            transform = transform.mul_mat4(&parent_transform);
            bone = parent_bone;
        } else {
            break;
        }
    }

    // Save the result in row-major order.
    Ok(transform.transpose().to_cols_array_2d())
}

fn animate_skel(
    anim: &AnimData,
    bones: &mut [AnimatedBone],
    frame: f32,
    meshes: &mut [RenderMesh],
) {
    for group in &anim.groups {
        match group.group_type {
            GroupType::Transform => {
                for node in &group.nodes {
                    if let Some(track) = node.tracks.first() {
                        if let TrackValues::Transform(values) = &track.values {
                            // TODO: Set the frame/interpolation?
                            // TODO: Faster to use SIMD types?
                            if let Some(bone) = bones.iter_mut().find(|b| b.bone.name == node.name)
                            {
                                apply_transform_track(frame, track, values, bone);
                            }
                        }
                    }
                }
            }
            // TODO: Handle other animation types?
            GroupType::Visibility => {
                for node in &group.nodes {
                    if let Some(track) = node.tracks.first() {
                        if let TrackValues::Boolean(values) = &track.values {
                            // TODO: Is this the correct way to process mesh names?
                            // TODO: Test visibility anims?
                            // Ignore the _VIS_....
                            // An Eye track toggles EyeL and EyeR?
                            for mesh in meshes.iter_mut().filter(|m| m.name.starts_with(&node.name))
                            {
                                // TODO: Share this between tracks?
                                let (current_frame, next_frame, factor) =
                                    frame_values(frame, track);
                                // dbg!(&node.name, values[current_frame]);
                                mesh.is_visible = values[current_frame];
                            }
                        }
                    }
                }
            }
            GroupType::Material => (),
            // TODO: Camera animations should apply to the scene camera?
            GroupType::Camera => (),
        }
    }
}

fn interp_quat(a: &Vector4, b: &Vector4, factor: f32) -> glam::Quat {
    glam::Quat::from_xyzw(a.x, a.y, a.z, a.w)
        .lerp(glam::Quat::from_xyzw(b.x, b.y, b.z, b.w), factor)
}

fn interp_vec3(a: &Vector3, b: &Vector3, factor: f32) -> glam::Vec3 {
    // TODO: SIMD type here?
    glam::Vec3::from(a.to_array()).lerp(glam::Vec3::from(b.to_array()), factor)
}

fn transform_to_mat4(transform: &AnimTransform) -> glam::Mat4 {
    let translation = glam::Mat4::from_translation(transform.translation);

    let rotation = glam::Mat4::from_quat(transform.rotation);

    // TODO: Scaling is more complicated and depends on the scale options.
    let scale = glam::Mat4::from_scale(transform.scale);

    // The application order is scale -> rotation -> translation.
    // The order is reversed here since glam is column-major.
    // TODO: Why do we transpose here?
    (translation * rotation * scale).transpose()
}

fn apply_transform_track(
    frame: f32,
    track: &ssbh_data::anim_data::TrackData,
    values: &[ssbh_data::anim_data::Transform],
    bone: &mut AnimatedBone,
) {
    let (current_frame, next_frame, factor) = frame_values(frame, track);

    let current = values[current_frame];
    let next = values[next_frame];

    // TODO: Override with rest pose based on transform flags?
    // TODO: Where to apply transform flags?
    bone.compensate_scale = track.scale_options.compensate_scale;
    bone.inherit_scale = track.scale_options.inherit_scale;

    bone.anim_transform = Some(AnimTransform {
        translation: interp_vec3(&current.translation, &next.translation, factor),
        rotation: interp_quat(&current.rotation, &next.rotation, factor),
        scale: interp_vec3(&current.scale, &next.scale, factor),
    });
}

fn frame_values(frame: f32, track: &ssbh_data::anim_data::TrackData) -> (usize, usize, f32) {
    // Force the frame to be in bounds.
    // TODO: Is this the correct way to handle single frame const animations?
    // TODO: Tests for interpolation?
    let current_frame = (frame.floor() as usize).clamp(0, track.values.len() - 1);
    let next_frame = (frame.ceil() as usize).clamp(0, track.values.len() - 1);
    // TODO: Not all animations interpolate?
    let factor = frame.fract();

    (current_frame, next_frame, factor)
}

#[cfg(test)]
mod tests {
    use approx::relative_eq;
    use ssbh_data::{
        anim_data::{GroupData, NodeData, ScaleOptions, TrackData, Transform, TransformFlags},
        skel_data::{BillboardType, BoneData},
    };

    use super::*;

    macro_rules! assert_matrix_relative_eq {
        ($a:expr, $b:expr) => {
            assert!(
                $a.iter()
                    .flatten()
                    .zip($b.iter().flatten())
                    .all(|(a, b)| relative_eq!(a, b, epsilon = 0.0001f32)),
                "Matrices not equal to within 0.0001.\nleft = {:?}\nright = {:?}",
                $a,
                $b
            );
        };
    }

    #[test]
    fn test() {
        // TODO: How to test animations?
        // Compare the matrices to the expected matrix (use approx).
        // Construct AnimData and SkelData?
        // Test scale inheritance, scale compensation, etc
        // Test with a three bone chain based on known test anims?
        // Cycle detection in the skeleton?
        // Out of range frame indices (negative, too large, etc)
        // Interpolation behavior
    }

    #[test]
    fn apply_animation_empty() {
        apply_animation(
            &SkelData {
                major_version: 1,
                minor_version: 0,
                bones: Vec::new(),
            },
            Some(&AnimData {
                major_version: 2,
                minor_version: 0,
                final_frame_index: 0.0,
                groups: Vec::new(),
            }),
            0.0,
            &mut [],
        );
    }

    #[test]
    fn apply_animation_single_animated_bone() {
        // Check that the appropriate bones are set.
        // Check the construction of transformation matrices.
        let transforms = apply_animation(
            &SkelData {
                major_version: 1,
                minor_version: 0,
                bones: vec![BoneData {
                    name: "A".to_string(),
                    // Start with the identity to make this simpler.
                    transform: [
                        [1.0, 0.0, 0.0, 0.0],
                        [0.0, 1.0, 0.0, 0.0],
                        [0.0, 0.0, 1.0, 0.0],
                        [0.0, 0.0, 0.0, 1.0],
                    ],
                    parent_index: None,
                    billboard_type: BillboardType::Disabled,
                }],
            },
            Some(&AnimData {
                major_version: 2,
                minor_version: 0,
                final_frame_index: 0.0,
                groups: vec![GroupData {
                    group_type: GroupType::Transform,
                    nodes: vec![NodeData {
                        name: "A".to_string(),
                        tracks: vec![TrackData {
                            name: "Transform".to_string(),
                            scale_options: ScaleOptions::default(),
                            values: TrackValues::Transform(vec![Transform {
                                scale: Vector3::new(1.0, 2.0, 3.0),
                                rotation: Vector4::new(1.0, 0.0, 0.0, 0.0),
                                translation: Vector3::new(4.0, 5.0, 6.0),
                            }]),
                            transform_flags: TransformFlags::default(),
                        }],
                    }],
                }],
            }),
            0.0,
            &mut [],
        );

        // TODO: Test the unused indices?
        // TODO: No transpose?
        assert_matrix_relative_eq!(
            [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, -2.0, 0.0, 0.0],
                [0.0, 0.0, -3.0, 0.0],
                [4.0, 5.0, 6.0, 1.0],
            ],
            transforms.transforms.transforms[0]
                // .transpose()
                .to_cols_array_2d()
        );
        assert_matrix_relative_eq!(
            [
                [1.0 / 1.0, 0.0, 0.0, 4.0 / -1.0],
                [0.0, -1.0 / 2.0, 0.0, 5.0 / 2.0],
                [0.0, 0.0, -1.0 / 3.0, 6.0 / 3.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
            transforms.transforms.transforms_inv_transpose[0]
                // .transpose()
                .to_cols_array_2d()
        );
        assert_matrix_relative_eq!(
            [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, -2.0, 0.0, 0.0],
                [0.0, 0.0, -3.0, 0.0],
                [4.0, 5.0, 6.0, 1.0],
            ],
            transforms.world_transforms[0]
                // .transpose()
                .to_cols_array_2d()
        );
    }
}
