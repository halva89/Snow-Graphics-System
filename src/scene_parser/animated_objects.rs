use crate::animation::AnimatedMesh;
use crate::animation::Keyframe;
use crate::scene_parser::SceneObject;
use crate::types::Color;
use crate::math::Vec3;
use crate::scene_parser::mesh_loader::load_obj;
use std::sync::atomic::{AtomicU32, Ordering};

static ANIM_COUNTER: AtomicU32 = AtomicU32::new(0);

pub fn parse_animated(lines: &[&str], i: &mut usize) -> Option<SceneObject> {
    let line = lines[*i].trim();

    if !line.starts_with("animate") {
        return None;
    }

    let parts: Vec<&str> = line.split_whitespace().collect();
    let shape_type = if parts.len() >= 2 { parts[1] } else { "triangle" };
    let mesh_path = if shape_type == "mesh" && parts.len() >= 3 {
        Some(parts[2].trim_matches('"').to_string())
    } else {
        None
    };

    let mut keyframes = Vec::new();
    *i += 1;

    while *i < lines.len() {
        let inner = lines[*i].trim();
        if inner == "}" {
            *i += 1;
            break;
        }

        if inner.is_empty() || inner.starts_with('#') {
            *i += 1;
            continue;
        }

        if inner.starts_with("animate") {
            break;
        }

        if inner.starts_with("keyframe") {
            let kf_parts: Vec<&str> = inner.split_whitespace().collect();
            if kf_parts.len() >= 2 {
                let time = kf_parts[1].parse::<f32>().unwrap_or(0.0);

                let mut pos = Vec3::zero();
                let mut color = Color::white();
                let mut consumed = 0;

                // Следующие строки — пока не встретится новый keyframe или закрывающая скобка
                while *i + 1 + consumed < lines.len() {
                    let next_i = *i + 1 + consumed;
                    let next_line = lines[next_i].trim();
                    if next_line.is_empty()
                        || next_line.starts_with("keyframe")
                        || next_line == "}"
                    {
                        break;
                    }
                    let sub_parts: Vec<&str> = next_line.split_whitespace().collect();

                    if sub_parts.is_empty() {
                        consumed += 1;
                        continue;
                    }

                    match sub_parts[0] {
                        "pos" | "position" => {
                            if sub_parts.len() >= 4 {
                                let x = sub_parts[1].parse().unwrap_or(0.0);
                                let y = sub_parts[2].parse().unwrap_or(0.0);
                                let z = sub_parts[3].parse().unwrap_or(0.0);
                                pos = Vec3::new(x, y, z);
                            } else if sub_parts.len() >= 3 {
                                let x = sub_parts[1].parse().unwrap_or(0.0);
                                let y = sub_parts[2].parse().unwrap_or(0.0);
                                pos = Vec3::new(x, y, 0.0);
                            }
                        }
                        "color" => {
                            if sub_parts.len() >= 4 {
                                color = Color::new(
                                    sub_parts[1].parse().unwrap_or(1.0),
                                    sub_parts[2].parse().unwrap_or(1.0),
                                    sub_parts[3].parse().unwrap_or(1.0),
                                );
                            }
                        }
                        _ => {
                            // fallback: если первое слово — не keyword, пробуем как raw числа x y z
                            let x = sub_parts[0].parse::<f32>();
                            if x.is_ok() {
                                let x = x.unwrap();
                                let y = if sub_parts.len() >= 2 {
                                    sub_parts[1].parse().unwrap_or(0.0)
                                } else {
                                    0.0
                                };
                                let z = if sub_parts.len() >= 3 {
                                    sub_parts[2].parse().unwrap_or(0.0)
                                } else {
                                    0.0
                                };
                                pos = Vec3::new(x, y, z);
                                // Если есть следующая строка и это не keyword — это цвет
                                if consumed + 1 < lines.len() - (*i + 1) {
                                    let color_next = lines[*i + 1 + consumed + 1].trim();
                                    let cp: Vec<&str> = color_next.split_whitespace().collect();
                                    if !color_next.starts_with("keyframe")
                                        && color_next != "}"
                                        && cp.len() >= 3
                                        && cp[0].parse::<f32>().is_ok()
                                    {
                                        color = Color::new(
                                            cp[0].parse().unwrap_or(1.0),
                                            cp[1].parse().unwrap_or(1.0),
                                            cp[2].parse().unwrap_or(1.0),
                                        );
                                        consumed += 1;
                                    }
                                }
                            }
                        }
                    }

                    consumed += 1;
                }

                keyframes.push(Keyframe::new(time, pos, color));
                *i += 1 + consumed;
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
<<<<<<< Updated upstream
=======
    let n = ANIM_COUNTER.fetch_add(1, Ordering::Relaxed);
    mesh.name = format!("anim_{}_{}", shape_type, n);
>>>>>>> Stashed changes
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
        "cube" => {
            mesh.vertices = crate::render::mesh::Mesh::cube(1.0).vertices;
            mesh.indices = crate::render::mesh::Mesh::cube(1.0).indices;
        }
        "sphere" => {
            let m = crate::render::mesh::Mesh::sphere(0.5, 16);
            mesh.vertices = m.vertices;
            mesh.indices = m.indices;
        }
        "mesh" => {
            if let Some(path) = &mesh_path {
                if let Some(loaded) = load_obj(path) {
                    mesh.vertices = loaded.mesh.vertices;
                    mesh.indices = loaded.mesh.indices;
                }
            }
            if mesh.vertices.is_empty() {
                mesh.vertices = vec![-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];
                mesh.indices = vec![0, 1, 2];
            }
        }
        _ => {
            mesh.vertices = vec![-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];
            mesh.indices = vec![0, 1, 2];
        }
    }

    Some(SceneObject::Animated(mesh))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_str(s: &str) -> SceneObject {
        let lines: Vec<&str> = s.lines().collect();
        let mut i = 0;
        parse_animated(&lines, &mut i).expect("should parse animated object")
    }

    #[test]
    fn test_parse_with_keywords() {
        let obj = parse_str(
            "animate circle\n\
             keyframe 0.0\n\
             pos 0.0 0.0 0.0\n\
             color 1.0 0.0 0.0\n\
             keyframe 2.0\n\
             pos 0.8 0.8 0.0\n\
             color 0.0 1.0 0.0\n\
             keyframe 4.0\n\
             pos 0.0 0.0 0.0\n\
             color 0.0 0.0 1.0",
        );
        let SceneObject::Animated(anim) = obj else { panic!("expected animated") };
        assert_eq!(anim.keyframes.len(), 3);
        assert_eq!(anim.keyframes[0].time, 0.0);
        assert_eq!(anim.keyframes[0].position, Vec3::new(0.0, 0.0, 0.0));
        assert_eq!(anim.keyframes[1].position, Vec3::new(0.8, 0.8, 0.0));
        assert_eq!(anim.keyframes[2].color, Color::new(0.0, 0.0, 1.0));
        assert_eq!(anim.duration, 4.0);
        assert_eq!(anim.vertices.len(), 3 * (16 + 2));
    }

    #[test]
    fn test_parse_raw_numbers_2d() {
        let obj = parse_str(
            "animate square\n\
             keyframe 0.0\n\
             0.0 0.0 0.0\n\
             1.0 0.0 0.0\n\
             keyframe 1.0\n\
             1.0 1.0 0.0\n\
             0.0 1.0 0.0",
        );
        let SceneObject::Animated(anim) = obj else { panic!("expected animated") };
        assert_eq!(anim.keyframes.len(), 2);
        assert_eq!(anim.keyframes[0].position, Vec3::new(0.0, 0.0, 0.0));
        assert_eq!(anim.keyframes[1].position, Vec3::new(1.0, 1.0, 0.0));
        assert_eq!(anim.keyframes[0].color, Color::new(1.0, 0.0, 0.0));
        assert_eq!(anim.keyframes[1].color, Color::new(0.0, 1.0, 0.0));
        assert!(
            anim.name.starts_with("anim_square_"),
            "name should start with anim_square_, got: {}",
            anim.name
        );
    }

    #[test]
    fn test_unique_names() {
        let obj1 = parse_str("animate circle\nkeyframe 0.0\npos 0 0 0\ncolor 1 1 1");
        let obj2 = parse_str("animate circle\nkeyframe 0.0\npos 0 0 0\ncolor 1 1 1");
        let SceneObject::Animated(a1) = obj1 else { panic!() };
        let SceneObject::Animated(a2) = obj2 else { panic!() };
        assert_ne!(a1.name, a2.name, "two animations of same shape must have unique names");
    }

    #[test]
    fn test_parse_color_optional() {
        let obj = parse_str("animate triangle\nkeyframe 0.0\npos 1.0 2.0 0.0");
        let SceneObject::Animated(anim) = obj else { panic!() };
        assert_eq!(anim.keyframes[0].position, Vec3::new(1.0, 2.0, 0.0));
        assert_eq!(anim.keyframes[0].color, Color::white());
    }

    #[test]
    fn test_animate_cube_is_3d() {
        let obj = parse_str(
            "animate cube\n\
             keyframe 0.0\n\
             pos 0 0 0\n\
             color 1 0 0\n\
             keyframe 1.0\n\
             pos 1 0 0\n\
             color 0 1 0",
        );
        let SceneObject::Animated(anim) = obj else { panic!() };
        // Cube has 8 vertices * 3 coords = 24 floats
        assert_eq!(anim.vertices.len(), 24, "cube should have 8 vertices x 3 coords");
        assert_eq!(anim.indices.len(), 36, "cube should have 36 indices (6 faces x 2 tris x 3 idx)");
    }

    #[test]
    fn test_animate_sphere_is_3d() {
        let obj = parse_str(
            "animate sphere\n\
             keyframe 0.0\n\
             pos 0 0 0\n\
             color 1 1 1",
        );
        let SceneObject::Animated(anim) = obj else { panic!() };
        // Sphere with segments=16, rings=8 has (8+1)*(16+1) vertices
        let expected_verts = (8 + 1) * (16 + 1) * 3;
        assert_eq!(anim.vertices.len(), expected_verts, "sphere should have proper 3D vertices");
    }

    #[test]
    fn test_animate_triangle_default_2d() {
        let obj = parse_str(
            "animate unknown\n\
             keyframe 0.0\n\
             pos 0 0 0\n\
             color 1 1 1",
        );
        let SceneObject::Animated(anim) = obj else { panic!() };
        // Default fallback is a 2D triangle: 3 vertices x 3 coords
        assert_eq!(anim.vertices.len(), 9, "unknown shape falls back to 2D triangle");
    }
}