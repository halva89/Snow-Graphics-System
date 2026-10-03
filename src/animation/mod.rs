pub mod keyframe;
pub mod interpolator;
pub mod animator;

pub use keyframe::Keyframe;
pub use animator::Animator;

#[derive(Debug, Clone)]
pub struct AnimatedMesh {
    pub vertices: Vec<f32>,
    pub normals: Vec<f32>,
    pub indices: Vec<u32>,
    pub keyframes: Vec<Keyframe>,
    pub duration: f32,
    pub name: String,
}

impl AnimatedMesh {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            normals: Vec::new(),
            indices: Vec::new(),
            keyframes: Vec::new(),
            duration: 1.0,
            name: String::new(),
        }
    }
}