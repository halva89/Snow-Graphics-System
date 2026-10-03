use crate::math::Mat4;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Uniforms {
    pub projection: Mat4,
    pub view: Mat4,
    pub model: Mat4,
    pub light_pos: [f32; 4],
    pub light_color: [f32; 4],
    pub ambient: [f32; 4],
    pub view_pos: [f32; 4],
    pub light_matrix: Mat4,
    pub shadow_params: [f32; 4],
}

impl Uniforms {
    pub fn new() -> Self {
        Self {
            projection: Mat4::identity(),
            view: Mat4::identity(),
            model: Mat4::identity(),
            light_pos: [5.0, 8.0, 6.0, 1.0],
            light_color: [1.0, 1.0, 1.0, 1.0],
            ambient: [0.18, 0.18, 0.18, 1.0],
            view_pos: [0.0, 0.0, 5.0, 1.0],
            light_matrix: Mat4::identity(),
            shadow_params: [0.0, 0.0, 0.0, 0.0],
        }
    }
}