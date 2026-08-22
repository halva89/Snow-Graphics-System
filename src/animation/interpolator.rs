use crate::types::Color;
use crate::math::Vec3;
use crate::animation::Keyframe;

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

pub fn lerp_vec3(a: &Vec3, b: &Vec3, t: f32) -> Vec3 {
    Vec3::new(
        lerp(a.x, b.x, t),
        lerp(a.y, b.y, t),
        lerp(a.z, b.z, t),
    )
}

pub fn lerp_color(a: &Color, b: &Color, t: f32) -> Color {
    Color::new(
        lerp(a.r, b.r, t),
        lerp(a.g, b.g, t),
        lerp(a.b, b.b, t),
    )
}

pub fn interpolate_keyframes(current: &Keyframe, next: &Keyframe, t: f32) -> (Vec3, Color) {
    let pos = lerp_vec3(&current.position, &next.position, t);
    let col = lerp_color(&current.color, &next.color, t);
    (pos, col)
}