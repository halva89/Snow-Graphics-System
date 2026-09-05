use rapier3d::prelude::*;

pub struct PhysicsWorld {
    pub gravity: Vector<f32>,
    pub integration_parameters: IntegrationParameters,
    pub physics_pipeline: PhysicsPipeline,
    pub island_manager: IslandManager,
    pub broad_phase: BroadPhase,
    pub narrow_phase: NarrowPhase,
    pub bodies: RigidBodySet,
    pub colliders: ColliderSet,
    pub impulse_joints: ImpulseJointSet,
    pub multibody_joints: MultibodyJointSet,
    pub ccd_solver: CCDSolver,
}

impl PhysicsWorld {
    pub fn new() -> Self {
<<<<<<< Updated upstream
        let gravity = vector![0.0, -9.81, 0.0];
        
        Self {
            gravity,
=======
        let mut world = Self {
            gravity: vector![0.0, -9.81, 0.0],
>>>>>>> Stashed changes
            integration_parameters: IntegrationParameters::default(),
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            bodies: RigidBodySet::new(),
            colliders: ColliderSet::new(),
            impulse_joints: ImpulseJointSet::new(),
            multibody_joints: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
        };
        // Add ground plane
        let ground = RigidBodyBuilder::fixed().build();
        let gh = world.bodies.insert(ground);
        let halfspace = ColliderBuilder::halfspace(Vector::y_axis()).build();
        world.colliders.insert_with_parent(halfspace, gh, &mut world.bodies);
        world
    }

<<<<<<< Updated upstream
=======
    pub fn add_body(&mut self, pos: Vector<f32>, half: Vector<f32>, mass: f32, restitution: f32, density: f32) -> RigidBodyHandle {
        println!("[Physics] add_body pos=({},{},{}) half=({},{},{}) mass={} rest={} density={}", pos.x, pos.y, pos.z, half.x, half.y, half.z, mass, restitution, density);
        if mass <= 0.0 && density <= 0.0 {
            let body = RigidBodyBuilder::fixed().translation(pos).build();
            let h = self.bodies.insert(body);
            let coll = ColliderBuilder::cuboid(half.x, half.y, half.z)
                .restitution(restitution)
                .build();
            self.colliders.insert_with_parent(coll, h, &mut self.bodies);
            h
        } else {
            let body = RigidBodyBuilder::dynamic()
                .translation(pos)
                .additional_mass(mass)
                .build();
            let h = self.bodies.insert(body);
            let coll = ColliderBuilder::cuboid(half.x, half.y, half.z)
                .restitution(restitution)
                .density(density)
                .build();
            self.colliders.insert_with_parent(coll, h, &mut self.bodies);
            h
        }
    }

>>>>>>> Stashed changes
    pub fn step(&mut self) {
        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.impulse_joints,
            &mut self.multibody_joints,
            &mut self.ccd_solver,
            None,
            &(),
            &(),
        );
    }

    pub fn add_static_box(&mut self, position: Vector<f32>, half_extents: Vector<f32>) {
        let collider = ColliderBuilder::cuboid(half_extents.x, half_extents.y, half_extents.z)
            .translation(position)
            .build();
        self.colliders.insert(collider);
    }

    pub fn add_dynamic_box(&mut self, position: Vector<f32>, half_extents: Vector<f32>) -> RigidBodyHandle {
        let body = RigidBodyBuilder::dynamic()
            .translation(position)
            .build();
        let body_handle = self.bodies.insert(body);
        
        let collider = ColliderBuilder::cuboid(half_extents.x, half_extents.y, half_extents.z)
            .build();
        self.colliders.insert_with_parent(collider, body_handle, &mut self.bodies);
        
        body_handle
    }
}

impl Default for PhysicsWorld {
    fn default() -> Self {
        Self::new()
    }
}