use crate::math::Mat4;

#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub model: Mat4,
    pub position: (f32, f32, f32),
    pub scale: (f32, f32, f32),
    pub rotation: (f32, f32, f32),
}

impl Transform {
    pub fn new() -> Self {
        Self {
            model: Mat4::identity(),
            position: (0.0, 0.0, 0.0),
            scale: (1.0, 1.0, 1.0),
            rotation: (0.0, 0.0, 0.0),
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.position = (x, y, z);
        self.update_model();
    }

    pub fn set_scale(&mut self, x: f32, y: f32, z: f32) {
        self.scale = (x, y, z);
        self.update_model();
    }

    pub fn set_rotation(&mut self, x: f32, y: f32, z: f32) {
        self.rotation = (x, y, z);
        self.update_model();
    }

    fn update_model(&mut self) {
        let (px, py, pz) = self.position;
        let (sx, sy, sz) = self.scale;
        let (rx, ry, rz) = self.rotation;

        let translate = Mat4::translation(px, py, pz);
        let scale = Mat4::scale(sx, sy, sz);
        let rotate = Mat4::rotation(rx, ry, rz);

        self.model = Mat4::multiply(&translate, &Mat4::multiply(&rotate, &scale));
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::new()
    }
}