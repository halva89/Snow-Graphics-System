use crate::render::mesh::Mesh;
use crate::types::Color;
use crate::math::Vec3;

pub struct LoadedMesh {
    pub mesh: Mesh,
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
}

pub fn load_obj(path: &str) -> Option<LoadedMesh> {
    let content = std::fs::read_to_string(path).ok()?;
    let mut pos_temp: Vec<[f32; 3]> = Vec::new();
    let mut ind_temp: Vec<u32> = Vec::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() { continue; }

        match parts[0] {
            "v" if parts.len() >= 4 => {
                let x: f32 = parts[1].parse().unwrap_or(0.0);
                let y: f32 = parts[2].parse().unwrap_or(0.0);
                let z: f32 = parts[3].parse().unwrap_or(0.0);
                pos_temp.push([x, y, z]);
            }
            "f" => {
                // Face: can be "f v1 v2 v3" or "f v1/t1 v2/t2 v3/t3" or "f v1//n1 v2//n2 v3//n3"
                for i in 1..parts.len() {
                    let idx_str = parts[i].split('/').next().unwrap_or("");
                    if let Ok(idx) = idx_str.parse::<u32>() {
                        ind_temp.push(idx.wrapping_sub(1));
                    }
                }
            }
            _ => {}
        }
    }

    if pos_temp.is_empty() || ind_temp.is_empty() {
        println!("[OBJ] Empty or invalid mesh in {}", path);
        return None;
    }

    let mut vertices = Vec::new();
    for p in &pos_temp {
        vertices.extend_from_slice(p);
    }

    let index_count = ind_temp.len() as u32;

    let mesh = Mesh {
        vertices,
        indices: ind_temp,
        color: Color::new(0.7, 0.7, 0.7),
        vertex_count: pos_temp.len() as u32,
        index_count,
    };

    println!("[OBJ] Loaded {} with {} vertices, {} triangles",
        path, mesh.vertex_count, mesh.index_count / 3);

    Some(LoadedMesh {
        mesh,
        position: Vec3::zero(),
        rotation: Vec3::zero(),
        scale: Vec3::one(),
    })
}