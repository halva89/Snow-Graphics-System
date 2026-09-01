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

    // 2D примитивы
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

        mesh.vertices.extend(&[cx, cy, 0.0]);
        for i in 0..=segments {
            let angle = (i as f32 / segments as f32) * std::f32::consts::TAU;
            mesh.vertices.extend(&[
                cx + radius * angle.cos(),
                cy + radius * angle.sin(),
                0.0,
            ]);
        }

        for i in 0..segments {
            let current = i + 1;
            let next = if i == segments - 1 { 1 } else { i + 2 };
            mesh.indices.extend(&[0, current, next]);
        }

        mesh.vertex_count = (mesh.vertices.len() / 3) as u32;
        mesh.index_count = mesh.indices.len() as u32;
        mesh
    }

    // 3D примитивы
    pub fn cube(size: f32) -> Self {
        let half = size / 2.0;
        let mut mesh = Self::new();
        
        // 8 вершин куба
        let v = [
            [-half, -half, -half], // 0
            [ half, -half, -half], // 1
            [ half,  half, -half], // 2
            [-half,  half, -half], // 3
            [-half, -half,  half], // 4
            [ half, -half,  half], // 5
            [ half,  half,  half], // 6
            [-half,  half,  half], // 7
        ];

        for vert in v.iter() {
            mesh.vertices.extend(&[vert[0], vert[1], vert[2]]);
        }

        // Индексы для 6 граней (по 2 треугольника на грань)
        let faces = [
            [0, 1, 2, 0, 2, 3], // front
            [4, 6, 5, 4, 7, 6], // back
            [1, 5, 6, 1, 6, 2], // right
            [0, 3, 7, 0, 7, 4], // left
            [3, 2, 6, 3, 6, 7], // top
            [0, 4, 5, 0, 5, 1], // bottom
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

    pub fn sphere(radius: f32, segments: u32) -> Self {
        let mut mesh = Self::new();
        let segments = segments.max(3);
        let rings = segments / 2;

        // Вершины
        for j in 0..=rings {
            let phi = (j as f32 / rings as f32) * std::f32::consts::PI;
            for i in 0..=segments {
                let theta = (i as f32 / segments as f32) * std::f32::consts::TAU;
                
                let x = radius * phi.sin() * theta.cos();
                let y = radius * phi.cos();
                let z = radius * phi.sin() * theta.sin();
                
                mesh.vertices.extend(&[x, y, z]);
            }
        }

        // Индексы
        for j in 0..rings {
            for i in 0..segments {
                let a = j * (segments + 1) + i;
                let b = a + segments + 1;
                let c = a + 1;
                let d = b + 1;

                mesh.indices.extend(&[a, b, c]);
                mesh.indices.extend(&[c, b, d]);
            }
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
}

impl Default for Mesh {
    fn default() -> Self {
        Self::new()
    }
}