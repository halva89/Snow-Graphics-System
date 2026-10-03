use main_frame::core::engine::Engine;

fn main() {
<<<<<<< Updated upstream
    let (settings, objects) = load_scene("assets/level1.sgss");
    let mut engine = Engine::new(settings);
    engine.add_objects(objects);
    engine.run();
=======
    let path = std::env::args().nth(1).unwrap_or_else(|| "assets/main.mff".to_string());
    Engine::run_from(path);
>>>>>>> Stashed changes
}