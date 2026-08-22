use crate::types::Color;
use crate::math::Vec3;

#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<f32>,
    pub indices: Vec<u32>,
    pub normals: Vec<f32>,
    pub uvs: Vec<f32>,
    pub color: Color,
    pub vertex_count: u32,
    pub index_count: u32,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            normals: Vec::new(),
            uvs: Vec::new(),
            color: Color::white(),
            vertex_count: 0,
            index_count: 0,
        }
    }

    pub fn triangle(v1: (f32, f32), v2: (f32, f32), v3: (f32, f32)) -> Self {
        let mut mesh = Self::new();
        mesh.vertices.extend(&[v1.0, v1.1, 0.0, v2.0, v2.1, 0.0, v3.0, v3.1, 0.0]);
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
            mesh.vertices.extend(&[cx + radius * angle.cos(), cy + radius * angle.sin(), 0.0]);
        }

        for i in 0..segments {
            let current = i + 1;
            let next = if i == segments - 1 { 1 } else { i + 2 };
            mesh.indices.extend(&[0, current, next]);
        }

        mesh.vertex_count = segments + 2;
        mesh.index_count = segments * 3;
        mesh
    }

    pub fn cube(size: f32) -> Self {
        let half = size / 2.0;
        let mut mesh = Self::new();

        // 8 вершин куба
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

        // 12 треугольников (6 граней по 2 треугольника)
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

    pub fn sphere(radius: f32, rings: u32, sectors: u32) -> Self {
        let mut mesh = Self::new();

        for i in 0..=rings {
            let theta = (i as f32 / rings as f32) * std::f32::consts::PI;
            let sin_theta = theta.sin();
            let cos_theta = theta.cos();

            for j in 0..=sectors {
                let phi = (j as f32 / sectors as f32) * std::f32::consts::TAU;
                let sin_phi = phi.sin();
                let cos_phi = phi.cos();

                let x = radius * sin_theta * cos_phi;
                let y = radius * cos_theta;
                let z = radius * sin_theta * sin_phi;

                mesh.vertices.extend(&[x, y, z]);
            }
        }

        for i in 0..rings {
            for j in 0..sectors {
                let a = i * (sectors + 1) + j;
                let b = i * (sectors + 1) + j + 1;
                let c = (i + 1) * (sectors + 1) + j;
                let d = (i + 1) * (sectors + 1) + j + 1;

                mesh.indices.extend(&[a, b, c]);
                mesh.indices.extend(&[b, d, c]);
            }
        }

        mesh.vertex_count = ((rings + 1) * (sectors + 1)) as u32;
        mesh.index_count = (rings * sectors * 6) as u32;
        mesh
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn with_normals(mut self) -> Self {
        // Простая генерация нормалей для каждого треугольника
        self.normals.clear();
        for i in (0..self.indices.len()).step_by(3) {
            let i0 = self.indices[i] as usize * 3;
            let i1 = self.indices[i + 1] as usize * 3;
            let i2 = self.indices[i + 2] as usize * 3;

            let v0 = Vec3::new(self.vertices[i0], self.vertices[i0 + 1], self.vertices[i0 + 2]);
            let v1 = Vec3::new(self.vertices[i1], self.vertices[i1 + 1], self.vertices[i1 + 2]);
            let v2 = Vec3::new(self.vertices[i2], self.vertices[i2 + 1], self.vertices[i2 + 2]);

            let edge1 = v1 - v0;
            let edge2 = v2 - v0;
            let normal = edge1.cross(&edge2).normalize();

            // Добавляем нормаль для каждой вершины
            for _ in 0..3 {
                self.normals.extend(&[normal.x, normal.y, normal.z]);
            }
        }
        self
    }

    pub fn merge(&mut self, other: &Mesh) {
        let base_vertex = self.vertex_count;
        let base_index = self.index_count;

        for vert in other.vertices.chunks_exact(3) {
            self.vertices.extend(vert);
        }
        for idx in &other.indices {
            self.indices.push(idx + base_vertex);
        }
        if let Some(normals) = other.normals.chunks_exact(3) {
            for normal in normals {
                self.normals.extend(normal);
            }
        }
        if let Some(uvs) = other.uvs.chunks_exact(2) {
            for uv in uvs {
                self.uvs.extend(uv);
            }
        }

        self.vertex_count += other.vertex_count;
        self.index_count += other.index_count;
    }

    pub fn apply_transform(&mut self, matrix: &crate::math::Mat4) {
        for i in (0..self.vertices.len()).step_by(3) {
            let v = Vec3::new(self.vertices[i], self.vertices[i + 1], self.vertices[i + 2]);
            let transformed = matrix.transform(&v);
            self.vertices[i] = transformed.x;
            self.vertices[i + 1] = transformed.y;
            self.vertices[i + 2] = transformed.z;
        }
    }

    pub fn as_vertices_slice(&self) -> &[f32] {
        &self.vertices
    }

    pub fn as_indices_slice(&self) -> &[u32] {
        &self.indices
    }

    pub fn as_normals_slice(&self) -> &[f32] {
        &self.normals
    }

    pub fn as_uvs_slice(&self) -> &[f32] {
        &self.uvs
    }
}

impl Default for Mesh {
    fn default() -> Self {
        Self::new()
    }
}