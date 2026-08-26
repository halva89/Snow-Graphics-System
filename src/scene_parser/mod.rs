pub mod scene_settings;
pub mod static_objects;
pub mod animated_objects;
pub mod mesh_loader;

pub use scene_settings::SceneSettings;
pub use static_objects::parse_static;
pub use animated_objects::parse_animated;

use crate::render::mesh::Mesh;
use crate::animation::AnimatedMesh;
use crate::types::Color;
use std::fs;

pub enum SceneObject {
    Static(Mesh),
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

    while i < lines.len() {
        let line = lines[i].trim();
        println!("[Parser] Line {}: '{}'", i, line);

        if line.is_empty() || line.starts_with('#') {
            i += 1;
            continue;
        }

        if line.starts_with("window") || line.starts_with("title") {
            settings = scene_settings::parse_settings(&lines, &mut i);
            println!("[Parser] Settings parsed: {}x{}", settings.width, settings.height);
            continue;
        }

        if line.starts_with("animate") {
            println!("[Parser] Found animate block");
            if let Some(obj) = animated_objects::parse_animated(&lines, &mut i) {
                objects.push(obj);
                println!("[Parser] Animated object added");
            }
            continue;
        }

        // ===== ОБРАБОТКА ВСЕХ ОБЪЕКТОВ =====
        let shape = line;
        
        if shape == "cube" {
            println!("[Parser] Found cube");
            if i + 2 >= lines.len() {
                println!("[Parser] Not enough lines for cube");
                i += 1;
                continue;
            }
            
            let size = lines[i + 1]
                .split_whitespace()
                .next()
                .unwrap_or("1.0")
                .parse::<f32>()
                .unwrap_or(1.0);
                
            let color = parse_color(lines[i + 2]);
            
            let mut mesh = Mesh::cube(size);
            mesh.color = color;
            i += 3;
            objects.push(SceneObject::Static(mesh));
            println!("[Parser] Cube added, size={}", size);
            continue;
        }

        if shape == "sphere" {
            println!("[Parser] Found sphere");
            if i + 3 >= lines.len() {
                println!("[Parser] Not enough lines for sphere");
                i += 1;
                continue;
            }
            
            let radius = lines[i + 1]
                .split_whitespace()
                .next()
                .unwrap_or("0.5")
                .parse::<f32>()
                .unwrap_or(0.5);
                
            let segments = lines[i + 2]
                .split_whitespace()
                .next()
                .unwrap_or("16")
                .parse::<u32>()
                .unwrap_or(16);
                
            let color = parse_color(lines[i + 3]);
            
            let mut mesh = Mesh::sphere(radius, segments);
            mesh.color = color;
            i += 4;
            objects.push(SceneObject::Static(mesh));
            println!("[Parser] Sphere added, radius={}, segments={}", radius, segments);
            continue;
        }

        // Остальные объекты через parse_static
        if let Some(obj) = parse_static(line, &lines, &mut i) {
            objects.push(obj);
            println!("[Parser] Static object added");
            continue;
        }

        println!("[Parser] Unknown line: {}", line);
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