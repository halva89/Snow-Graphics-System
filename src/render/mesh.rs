use crate::types::Color;

#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<f32>,
    pub indices: Vec<u32>,
    pub color: Color,
    pub vertex_count: u32,
    pub index_count: u32,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            color: Color::white(),
            vertex_count: 0,
            index_count: 0,
        }
    }

    pub fn triangle(v1: (f32, f32), v2: (f32, f32), v3: (f32, f32)) -> Self {
        let mut mesh = Self::new();
        mesh.vertices.extend(&[
            v1.0, v1.1, 0.0,
            v2.0, v2.1, 0.0,
            v3.0, v3.1, 0.0,
        ]);
        mesh.indices.extend(&[0, 1, 2]);
        mesh.vertex_count = 3;
        mesh.index_count = 3;
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
        mesh.vertex_count = 4;
        mesh.index_count = 6;
        mesh
    }

    pub fn circle(cx: f32, cy: f32, radius: f32, segments: u32) -> Self {
        let mut mesh = Self::new();
        let segments = segments.max(3);

        // Центр
        mesh.vertices.extend(&[cx, cy, 0.0]);

        // Вершины по окружности
        for i in 0..=segments {
            let angle = (i as f32 / segments as f32) * std::f32::consts::TAU;
            mesh.vertices.extend(&[
                cx + radius * angle.cos(),
                cy + radius * angle.sin(),
                0.0,
            ]);
        }

        // Индексы треугольников (веер)
        for i in 0..segments {
            let current = i + 1;
            let next = if i == segments - 1 { 1 } else { i + 2 };
            mesh.indices.extend(&[0, current, next]);
        }

        mesh.vertex_count = (mesh.vertices.len() / 3) as u32;
        mesh.index_count = mesh.indices.len() as u32;
        
        mesh
    }

    pub fn from_animated(anim: crate::animation::AnimatedMesh) -> Self {
        let mut mesh = Self::new();
        mesh.vertices = anim.vertices;
        mesh.indices = anim.indices;
        mesh.vertex_count = (mesh.vertices.len() / 3) as u32;
        mesh.index_count = mesh.indices.len() as u32;
        mesh
    }

    pub fn cube(size: f32) -> Self {
        let half = size / 2.0;
        let mut mesh = Self::new();
        
        let v = [
            [-half, -half, -half],
            [ half, -half, -half],
            [ half,  half, -half],
            [-half,  half, -half],
            [-half, -half,  half],
            [ half, -half,  half],
            [ half,  half,  half],
            [-half,  half,  half],
        ];

        for vert in v.iter() {
            mesh.vertices.extend(&[vert[0], vert[1], vert[2]]);
        }

        let faces = [
            [0, 1, 2, 0, 2, 3],
            [4, 6, 5, 4, 7, 6],
            [1, 5, 6, 1, 6, 2],
            [0, 3, 7, 0, 7, 4],
            [3, 2, 6, 3, 6, 7],
            [0, 4, 5, 0, 5, 1],
        ];

        for face in faces.iter() {
            for &idx in face.iter() {
                mesh.indices.push(idx);
            }
        }

        mesh.vertex_count = 8;
        mesh.index_count = 36;
        mesh
    }
}

impl Default for Mesh {
    fn default() -> Self {
        Self::new()
    }
}