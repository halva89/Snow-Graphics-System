use crate::types::Color;

#[derive(Debug, Clone)]
pub struct Keyframe {
    pub time: f32,
    pub position: (f32, f32),
    pub color: Color,
}

#[derive(Debug, Clone)]
pub struct AnimatedMesh {
    pub vertices: Vec<f32>,
    pub indices: Vec<u32>,
    pub keyframes: Vec<Keyframe>,
    pub duration: f32,
}

#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<f32>,
    pub indices: Vec<u32>,
    pub color: Color,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            color: Color::white(),
        }
    }

    pub fn triangle(v1: (f32, f32), v2: (f32, f32), v3: (f32, f32)) -> Self {
        let mut mesh = Self::new();
        mesh.vertices.extend(&[v1.0, v1.1, 0.0, v2.0, v2.1, 0.0, v3.0, v3.1, 0.0]);
        mesh.indices.extend(&[0, 1, 2]);
        mesh
    }

    pub fn square(x: f32, y: f32, size: f32) -> Self {
        let half = size / 2.0;
        let mut mesh = Self::new();
        mesh.vertices.extend(&[
            x - half, y - half, 0.0,
            x + half, y - half, 0.0,
            x + half, y + half, 0.0,
            x - half, y + half, 0.0,
        ]);
        mesh.indices.extend(&[0, 1, 2, 0, 2, 3]);
        mesh
    }

    pub fn circle(cx: f32, cy: f32, radius: f32, segments: u32) -> Self {
        let mut mesh = Self::new();
        let segments = segments.max(3);
        
        mesh.vertices.extend(&[cx, cy, 0.0]);

        for i in 0..=segments {
            let angle = (i as f32 / segments as f32) * std::f32::consts::TAU;
            let x = cx + radius * angle.cos();
            let y = cy + radius * angle.sin();
            mesh.vertices.extend(&[x, y, 0.0]);
        }

        for i in 0..segments {
            let current = i + 1;
            let next = if i == segments - 1 { 1 } else { i + 2 };
            mesh.indices.extend(&[0, current, next]);
        }

        mesh
    }
}

impl AnimatedMesh {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            keyframes: Vec::new(),
            duration: 1.0,
        }
    }

    pub fn sample(&self, time: f32) -> (f32, f32, Color) {
        if self.keyframes.is_empty() {
            return (0.0, 0.0, Color::white());
        }
        
        let time = time % self.duration;
        
        for i in 0..self.keyframes.len() {
            let current = &self.keyframes[i];
            let next = &self.keyframes[(i + 1) % self.keyframes.len()];
            
            if time >= current.time && time < next.time {
                let t = (time - current.time) / (next.time - current.time);
                let x = current.position.0 + (next.position.0 - current.position.0) * t;
                let y = current.position.1 + (next.position.1 - current.position.1) * t;
                let r = current.color.r + (next.color.r - current.color.r) * t;
                let g = current.color.g + (next.color.g - current.color.g) * t;
                let b = current.color.b + (next.color.b - current.color.b) * t;
                return (x, y, Color::new(r, g, b));
            }
        }
        
        (0.0, 0.0, Color::white())
    }
}