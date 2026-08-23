use snow_graphics::core::engine::Engine;
use snow_graphics::scene_parser::load_scene;

fn main() {
    let (settings, objects) = load_scene("assets/level1.sgss");
    let mut engine = Engine::new(settings);
    engine.add_objects(objects);
    engine.run();
}