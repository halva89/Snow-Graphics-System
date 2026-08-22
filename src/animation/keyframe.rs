use crate::types::Color;
use crate::math::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Keyframe {
    pub time: f32,
    pub position: Vec3,
    pub color: Color,
}

impl Keyframe {
    pub fn new(time: f32, position: Vec3, color: Color) -> Self {
        Self { time, position, color }
    }
}