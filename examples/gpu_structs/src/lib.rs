use ggmath::{matrix::f32_aliases::*, vector::f32_aliases::*};

/// To send vertex buffers to the GPU,
/// you need to first send the GPU the layout of the buffer's verticies.
///
/// Because you specify the layout, the GPU doesn't expect a specific one.
/// Because of this, there are no layout rules to follow.
/// It is thus optimal to store the vectors as VecPacked to save space.
#[repr(C)]
pub struct Vertex {
    pos: FVec3P,
    normal: FVec3P,
    uv: FVec2P,
}

/// When sending uniforms to the gpu,
/// they have to align perfectly with their shading language counterpart,
/// Both type-wise and layout-wise.
///
/// Because of this there are strict layout rules that we need to follow.
/// in uniforms, vectors are required to follow the exact alignment rules of VecAligned,
/// so we will use that.
#[repr(C)]
pub struct UniformCamera {
    world_to_cam: FMat4C,
}
