use crate::mesh::Mesh;
use crate::types::Color;
use crate::math::Vec3;
use std::fs;

pub struct LoadedMesh {
    pub mesh: Mesh,
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
}

pub fn load_obj(path: &str) -> Option<LoadedMesh> {
    // Пока заглушка — полная реализация будет в Beryllium
    // Возвращаем куб как пример
    let mut mesh = Mesh::cube(1.0);
    mesh.color = Color::new(1.0, 0.5, 0.2);

    Some(LoadedMesh {
        mesh,
        position: Vec3::zero(),
        rotation: Vec3::zero(),
        scale: Vec3::one(),
    })
}

pub fn load_gltf(path: &str) -> Option<LoadedMesh> {
    // Пока заглушка — полная реализация будет в Beryllium
    load_obj(path)
}