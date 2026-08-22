use crate::core::object::Object;
use crate::scene_parser::SceneObject;

pub struct Scene {
    objects: Vec<Object>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add_object(&mut self, obj: SceneObject) {
        self.objects.push(Object::from(obj));
    }

    pub fn get_objects(&self) -> &Vec<Object> {
        &self.objects
    }

    pub fn get_objects_mut(&mut self) -> &mut Vec<Object> {
        &mut self.objects
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn len(&self) -> usize {
        self.objects.len()
    }

    pub fn is_empty(&self) -> bool {
        self.objects.is_empty()
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}