use crate::render::mesh::Mesh;
use crate::scene_parser::SceneObject;
use crate::types::Color;

pub fn parse_static(line: &str, lines: &[&str], i: &mut usize) -> Option<SceneObject> {
    if line == "triangle" {
        if *i + 4 >= lines.len() {
            return None;
        }
        let v1 = parse_vec2(lines[*i + 1]);
        let v2 = parse_vec2(lines[*i + 2]);
        let v3 = parse_vec2(lines[*i + 3]);
        let color = parse_color(lines[*i + 4]);
        let mut mesh = Mesh::triangle(v1, v2, v3);
        mesh.color = color;
        *i += 5;
        return Some(SceneObject::Static(mesh));
    }

    if line == "square" {
        if *i + 2 >= lines.len() {
            return None;
        }
        let parts: Vec<&str> = lines[*i + 1].split_whitespace().collect();
        if parts.len() >= 3 {
            let x = parts[0].parse().unwrap_or(0.0);
            let y = parts[1].parse().unwrap_or(0.0);
            let size = parts[2].parse().unwrap_or(0.5);
            let color = parse_color(lines[*i + 2]);
            let mut mesh = Mesh::square(x, y, size);
            mesh.color = color;
            *i += 3;
            return Some(SceneObject::Static(mesh));
        }
    }

    if line == "circle" {
        if *i + 2 >= lines.len() {
            return None;
        }
        let parts: Vec<&str> = lines[*i + 1].split_whitespace().collect();
        if parts.len() >= 4 {
            let cx = parts[0].parse().unwrap_or(0.0);
            let cy = parts[1].parse().unwrap_or(0.0);
            let radius = parts[2].parse().unwrap_or(0.3);
            let segments = parts[3].parse().unwrap_or(16);
            let color = parse_color(lines[*i + 2]);
            let mut mesh = Mesh::circle(cx, cy, radius, segments);
            mesh.color = color;
            *i += 3;
            return Some(SceneObject::Static(mesh));
        }
    }

    None
}

fn parse_vec2(line: &str) -> (f32, f32) {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() >= 2 {
        (parts[0].parse().unwrap_or(0.0), parts[1].parse().unwrap_or(0.0))
    } else {
        (0.0, 0.0)
    }
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