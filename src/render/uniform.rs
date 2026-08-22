use crate::math::Mat4;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Uniforms {
    pub model: Mat4,
    pub view: Mat4,
    pub projection: Mat4,
}

impl Uniforms {
    pub fn new() -> Self {
        Self {
            model: Mat4::identity(),
            view: Mat4::identity(),
            projection: Mat4::identity(),
        }
    }

    pub fn as_slice(&self) -> &[f32; 48] {
        unsafe { &*(self as *const _ as *const [f32; 48]) }
    }
}