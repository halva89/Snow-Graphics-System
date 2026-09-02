pub mod world;
pub mod body;

pub use world::PhysicsWorld;
pub use body::RigidBody;

#[derive(Clone, Copy)]
pub struct PhysicsProps {
    pub mass: f32,
    pub restitution: f32,
    pub density: f32,
}

impl PhysicsProps {
    pub fn new() -> Self {
        Self { mass: 1.0, restitution: 0.0, density: 1.0 }
    }
}

impl Default for PhysicsProps {
    fn default() -> Self { Self::new() }
}