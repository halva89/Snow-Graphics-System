use crate::math::Mat4;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Uniforms {
    pub projection: Mat4,
    pub view: Mat4,
    pub model: Mat4,
}

impl Uniforms {
    pub fn new() -> Self {
        Self {
            projection: Mat4::identity(),
            view: Mat4::identity(),
            model: Mat4::identity(),
        }
    }
}