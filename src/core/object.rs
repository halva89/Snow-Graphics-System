use crate::core::transform::Transform;
use crate::render::mesh::Mesh;
use crate::scene_parser::SceneObject;
use crate::types::Color;

pub struct Object {
    pub mesh: Mesh,
    pub transform: Transform,
    pub is_animated: bool,
    pub color: Color,
    pub name: String,
}

impl Object {
    pub fn from(scene_obj: SceneObject) -> Self {
        match scene_obj {
            SceneObject::Static(mesh, position) => {
                let color = mesh.color;
                let mut transform = Transform::default();
                transform.set_position(position.x, position.y, position.z);
                Self {
                    mesh,
                    transform,
                    is_animated: false,
                    color,
                    name: String::from("Static"),
                }
            }
            SceneObject::Animated(animated_mesh) => {
                let color = if !animated_mesh.keyframes.is_empty() {
                    animated_mesh.keyframes[0].color
                } else {
                    Color::white()
                };
                Self {
                    mesh: Mesh::from_animated(animated_mesh),
                    transform: Transform::default(),
                    is_animated: true,
                    color,
                    name: String::from("Animated"),
                }
            }
        }
    }
    pub fn new(mesh: Mesh, transform: Transform) -> Self {
        Self {
            mesh,
            transform,
            is_animated: false,
            color: Color::white(),
            name: String::from("Object"),
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.transform.set_position(x, y, z);
    }

    pub fn set_scale(&mut self, x: f32, y: f32, z: f32) {
        self.transform.set_scale(x, y, z);
    }

    pub fn set_rotation(&mut self, x: f32, y: f32, z: f32) {
        self.transform.set_rotation(x, y, z);
    }
}