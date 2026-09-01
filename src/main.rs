use snow_graphics::core::engine::Engine;
use snow_graphics::scene_parser::load_scene;
use snow_graphics::physics::PhysicsWorld;
use rapier3d::na::vector;  // <-- ЭТО

fn main() {
    let (settings, objects) = load_scene("assets/level1.sgss");
    let mut engine = Engine::new(settings);
    engine.add_objects(objects);
    
    let mut physics = PhysicsWorld::new();
    let _box = physics.add_dynamic_box(vector![0.0, 2.0, 0.0], vector![0.5, 0.5, 0.5]);
    
    engine.run();
}