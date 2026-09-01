pub mod scene_settings;
pub mod static_objects;
pub mod animated_objects;
pub mod mesh_loader;

pub use scene_settings::{SceneSettings, RenderMode};
pub use static_objects::parse_static;
pub use animated_objects::parse_animated;

use crate::render::mesh::Mesh;
use crate::animation::AnimatedMesh;
use crate::types::Color;
use crate::math::Vec3;
use std::fs;

pub enum SceneObject {
    Static(Mesh, Vec3),
    Animated(AnimatedMesh),
}

pub fn load_scene(path: &str) -> (SceneSettings, Vec<SceneObject>) {
    println!("[Parser] Loading scene: {}", path);

    let content = fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("Scene file not found: {}", path));

    let lines: Vec<&str> = content.lines().collect();
    println!("[Parser] Read {} lines", lines.len());

    let mut settings = SceneSettings::default();
    let mut objects = Vec::new();
    let mut i = 0;
    let mut pending_pos: Option<Vec3> = None;

    while i < lines.len() {
        let line = lines[i].trim();
        println!("[Parser] Line {}: '{}'", i, line);

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

        if line.starts_with("position") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 {
                let x = parts[1].parse().unwrap_or(0.0);
                let y = parts[2].parse().unwrap_or(0.0);
                let z = parts[3].parse().unwrap_or(0.0);
                pending_pos = Some(Vec3::new(x, y, z));
            }
            i += 1;
            continue;
        }

        let pos = pending_pos.take().unwrap_or(Vec3::zero());
        let shape = line;

        if shape == "cube" {
            if i + 2 >= lines.len() { i += 1; continue; }
            let size = lines[i + 1].split_whitespace().next().unwrap_or("1.0").parse().unwrap_or(1.0);
            let color = parse_color(lines[i + 2]);
            let mut mesh = Mesh::cube(size);
            mesh.color = color;
            i += 3;
            objects.push(SceneObject::Static(mesh, pos));
            continue;
        }

        if shape == "sphere" {
            if i + 3 >= lines.len() { i += 1; continue; }
            let radius = lines[i + 1].split_whitespace().next().unwrap_or("0.5").parse().unwrap_or(0.5);
            let segments = lines[i + 2].split_whitespace().next().unwrap_or("16").parse().unwrap_or(16);
            let color = parse_color(lines[i + 3]);
            let mut mesh = Mesh::sphere(radius, segments);
            mesh.color = color;
            i += 4;
            objects.push(SceneObject::Static(mesh, pos));
            continue;
        }

        if let Some(obj) = parse_static(line, &lines, &mut i, pos) {
            objects.push(obj);
            continue;
        }

        i += 1;
    }

    println!("[Parser] Loaded {} objects", objects.len());
    (settings, objects)
}

fn parse_color(line: &str) -> Color {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() >= 3 {
        Color::new(
            parts[0].parse().unwrap_or(1.0),
            parts[1].parse().unwrap_or(1.0),
            parts[2].parse().unwrap_or(1.0),
        )
    } else {
        Color::white()
    }
}