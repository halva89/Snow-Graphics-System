#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    pub fn red() -> Self { Self::new(1.0, 0.0, 0.0) }
    pub fn green() -> Self { Self::new(0.0, 1.0, 0.0) }
    pub fn blue() -> Self { Self::new(0.0, 0.0, 1.0) }
    pub fn white() -> Self { Self::new(1.0, 1.0, 1.0) }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle {
    pub vertices: [(f32, f32); 3],
    pub color: Color,
}

impl Triangle {
    pub fn new(v1: (f32, f32), v2: (f32, f32), v3: (f32, f32), color: Color) -> Self {
        Self {
            vertices: [v1, v2, v3],
            color,
        }
    }
}