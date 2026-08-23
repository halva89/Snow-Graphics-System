use crate::render::mesh::Mesh;
use crate::types::Color;
use crate::math::Vec3;

pub struct LoadedMesh {
    pub mesh: Mesh,
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
}

pub fn load_obj(_path: &str) -> Option<LoadedMesh> {
    let mut mesh = Mesh::cube(1.0);
    mesh.color = Color::new(1.0, 0.5, 0.2);

    Some(LoadedMesh {
        mesh,
        position: Vec3::zero(),
        rotation: Vec3::zero(),
        scale: Vec3::one(),
    })
}