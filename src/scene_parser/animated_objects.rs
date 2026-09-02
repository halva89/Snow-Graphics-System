use crate::animation::AnimatedMesh;
use crate::animation::Keyframe;
use crate::scene_parser::SceneObject;
use crate::types::Color;
use crate::math::Vec3;

pub fn parse_animated(lines: &[&str], i: &mut usize) -> Option<SceneObject> {
    let line = lines[*i].trim();

    if !line.starts_with("animate") {
        return None;
    }

    let parts: Vec<&str> = line.split_whitespace().collect();
    let shape_type = if parts.len() >= 2 { parts[1] } else { "triangle" };

    let mut keyframes = Vec::new();
    *i += 1;

    while *i < lines.len() {
        let inner = lines[*i].trim();
        if inner == "}" {
            *i += 1;
            break;
        }

        if inner.starts_with("keyframe") {
            let kf_parts: Vec<&str> = inner.split_whitespace().collect();
            if kf_parts.len() >= 2 {
                let time = kf_parts[1].parse::<f32>().unwrap_or(0.0);

                let pos_line = lines[*i + 1].trim();
                let pos_parts: Vec<&str> = pos_line.split_whitespace().collect();
                let pos = if pos_parts.len() >= 2 {
                    let x = pos_parts[0].parse().unwrap_or(0.0);
                    let y = pos_parts[1].parse().unwrap_or(0.0);
                    let z = if pos_parts.len() >= 3 {
                        pos_parts[2].parse().unwrap_or(0.0)
                    } else {
                        0.0
                    };
                    Vec3::new(x, y, z)
                } else {
                    Vec3::zero()
                };

                let color_line = lines[*i + 2].trim();
                let color = parse_color(color_line);

                keyframes.push(Keyframe::new(time, pos, color));
                *i += 3;
            } else {
                *i += 1;
            }
        } else {
            *i += 1;
        }
    }

    if keyframes.is_empty() {
        return None;
    }

    let mut mesh = AnimatedMesh::new();
    mesh.name = format!("anim_{}", shape_type);
    mesh.keyframes = keyframes;
    mesh.duration = mesh.keyframes.last().unwrap().time;

    match shape_type {
        "square" => {
            mesh.vertices = vec![-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.5, 0.5, 0.0, -0.5, 0.5, 0.0];
            mesh.indices = vec![0, 1, 2, 0, 2, 3];
        }
        "circle" => {
            let segments = 16;
            mesh.vertices.push(0.0);
            mesh.vertices.push(0.0);
            mesh.vertices.push(0.0);
            for j in 0..=segments {
                let angle = (j as f32 / segments as f32) * std::f32::consts::TAU;
                mesh.vertices.push(0.5 * angle.cos());
                mesh.vertices.push(0.5 * angle.sin());
                mesh.vertices.push(0.0);
            }
            for j in 0..segments {
                let current = j + 1;
                let next = if j == segments - 1 { 1 } else { j + 2 };
                mesh.indices.push(0);
                mesh.indices.push(current);
                mesh.indices.push(next);
            }
        }
        _ => {
            mesh.vertices = vec![-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];
            mesh.indices = vec![0, 1, 2];
        }
    }

    Some(SceneObject::Animated(mesh))
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