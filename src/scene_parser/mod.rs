pub mod scene_settings;
pub mod static_objects;
pub mod animated_objects;
pub mod mesh_loader;

pub use scene_settings::SceneSettings;
pub use static_objects::parse_static;
pub use animated_objects::parse_animated;

use crate::render::mesh::Mesh;
use crate::animation::AnimatedMesh;
use std::fs;

pub enum SceneObject {
    Static(Mesh),
    Animated(AnimatedMesh),
}

pub fn load_scene(path: &str) -> (SceneSettings, Vec<SceneObject>) {
    let content = fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("Scene file not found: {}", path));

    let lines: Vec<&str> = content.lines().collect();
    let mut settings = SceneSettings::default();
    let mut objects = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();

        if line.is_empty() || line.starts_with('#') {
            i += 1;
            continue;
        }

        if line.starts_with("window") || line.starts_with("title") {
            settings = scene_settings::parse_settings(&lines, &mut i);
            continue;
        }

        if line.starts_with("animate") {
            if let Some(obj) = animated_objects::parse_animated(&lines, &mut i) {
                objects.push(obj);
            }
            continue;
        }

        if let Some(obj) = static_objects::parse_static(line, &lines, &mut i) {
            objects.push(obj);
        }

        i += 1;
    }

    (settings, objects)
}