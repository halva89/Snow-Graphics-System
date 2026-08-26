use rapier3d::dynamics::RigidBodyHandle;
use rapier3d::prelude::*;

pub struct RigidBody {
    pub handle: RigidBodyHandle,
}

impl RigidBody {
    pub fn new(handle: RigidBodyHandle) -> Self {
        Self { handle }
    }

    pub fn get_position(&self, bodies: &RigidBodySet) -> Vector<f32> {
        let body = bodies.get(self.handle).unwrap();
        *body.translation()
    }

    pub fn set_position(&self, position: Vector<f32>, bodies: &mut RigidBodySet) {
        let body = bodies.get_mut(self.handle).unwrap();
        body.set_translation(position, true);
    }
}