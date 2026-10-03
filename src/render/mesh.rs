use crate::types::Color;

#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<f32>,
    pub normals: Vec<f32>,
    pub uvs: Vec<f32>,
    pub indices: Vec<u32>,
    pub color: Color,
    pub vertex_count: u32,
    pub index_count: u32,
    pub has_uv: bool,
    pub texture: Option<String>,
    pub texture_mode: TextureMode,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextureMode {
    Expand,
    Fill,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            normals: Vec::new(),
            uvs: Vec::new(),
            indices: Vec::new(),
            color: Color::white(),
            vertex_count: 0,
            index_count: 0,
            has_uv: false,
            texture: None,
            texture_mode: TextureMode::Expand,
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
        mesh.normals.extend(&[0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0]);
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
        mesh.normals.extend(&[0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0]);
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
        let n = segments as usize + 2;
        mesh.normals.extend(std::iter::repeat([0.0, 0.0, 1.0]).take(n).flatten());

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

        // 6 граней, каждая 4 вершины с собственными UV (24 вершины, 36 индексов)
        let faces: [([f32; 3], [f32; 3], [f32; 3], [f32; 3], [f32; 2], [f32; 2], [f32; 2], [f32; 2], [u32; 6]); 6] = [
            // front (z=-half): v0 = {-half,-half,-half} ...
            ([ -half, -half, -half], [ half, -half, -half], [ half,  half, -half], [-half,  half, -half], [0.0,1.0],[1.0,1.0],[1.0,0.0],[0.0,0.0], [0,1,2, 0,2,3]),
            // back (z=+half)
            ([ half, -half,  half], [-half, -half,  half], [-half,  half,  half], [ half,  half,  half], [0.0,1.0],[1.0,1.0],[1.0,0.0],[0.0,0.0], [4,5,6, 4,6,7]),
            // right (x=+half)
            ([ half, -half, -half], [ half, -half,  half], [ half,  half,  half], [ half,  half, -half], [0.0,1.0],[1.0,1.0],[1.0,0.0],[0.0,0.0], [8,9,10, 8,10,11]),
            // left (x=-half)
            ([-half, -half,  half], [-half, -half, -half], [-half,  half, -half], [-half,  half,  half], [0.0,1.0],[1.0,1.0],[1.0,0.0],[0.0,0.0], [12,13,14, 12,14,15]),
            // top (y=+half)
            ([-half,  half, -half], [ half,  half, -half], [ half,  half,  half], [-half,  half,  half], [0.0,1.0],[1.0,1.0],[1.0,0.0],[0.0,0.0], [16,17,18, 16,18,19]),
            // bottom (y=-half)
            ([-half, -half,  half], [ half, -half,  half], [ half, -half, -half], [-half, -half, -half], [0.0,1.0],[1.0,1.0],[1.0,0.0],[0.0,0.0], [20,21,22, 20,22,23]),
        ];

        // Нормали граней: передняя(z=-), задняя(z=+), правая(x=+), левая(x=-), верх(y=+), низ(y=-)
        let face_normals: [[f32; 3]; 6] = [
            [0.0, 0.0, -1.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, -1.0, 0.0],
        ];

        for (fi, face) in faces.iter().enumerate() {
            mesh.vertices.extend_from_slice(&[face.0[0], face.0[1], face.0[2], face.1[0], face.1[1], face.1[2], face.2[0], face.2[1], face.2[2], face.3[0], face.3[1], face.3[2]]);
            let nrm = face_normals[fi];
            mesh.normals.extend_from_slice(&[nrm[0], nrm[1], nrm[2], nrm[0], nrm[1], nrm[2], nrm[0], nrm[1], nrm[2], nrm[0], nrm[1], nrm[2]]);
            mesh.uvs.extend_from_slice(&[face.4[0], face.4[1], face.5[0], face.5[1], face.6[0], face.6[1], face.7[0], face.7[1]]);
            // Индексы в литерале уже абсолютные (0..23), добавлять idx_base не нужно
            mesh.indices.extend_from_slice(&face.8);
        }

        mesh.vertex_count = 24;
        mesh.index_count = 36;
        mesh.has_uv = true;
        mesh
    }

    pub fn sphere(radius: f32, segments: u32) -> Self {
        let mut mesh = Self::new();
        let segments = segments.max(3);
        let rings = segments / 2;

        // Вершины
        for j in 0..=rings {
            let phi = (j as f32 / rings as f32) * std::f32::consts::PI;
            let v = j as f32 / rings as f32;
            for i in 0..=segments {
                let theta = (i as f32 / segments as f32) * std::f32::consts::TAU;
                let u = i as f32 / segments as f32;

                let x = radius * phi.sin() * theta.cos();
                let y = radius * phi.cos();
                let z = radius * phi.sin() * theta.sin();

                mesh.vertices.extend(&[x, y, z]);
                mesh.normals.extend(&[x / radius, y / radius, z / radius]);
                mesh.uvs.extend(&[u, v]);
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
        mesh.has_uv = !mesh.uvs.is_empty();
        mesh
    }

    pub fn from_animated(anim: crate::animation::AnimatedMesh) -> Self {
        let mut mesh = Self::new();
        mesh.vertices = anim.vertices;
        mesh.indices = anim.indices;
        // Built-in shapes carry correct (outward) normals; only fall back to
        // recomputation when a mesh has none (e.g. arbitrary obj without normals).
        mesh.normals = if anim.normals.len() == mesh.vertices.len() {
            anim.normals
        } else {
            Self::compute_smooth_normals(&mesh.vertices, &mesh.indices)
        };
        mesh.vertex_count = (mesh.vertices.len() / 3) as u32;
        mesh.index_count = mesh.indices.len() as u32;
        mesh
    }

    // Плавные (усреднённые по площади) нормали для произвольной сетки треугольников
    pub fn compute_smooth_normals(vertices: &[f32], indices: &[u32]) -> Vec<f32> {
        let vcount = vertices.len() / 3;
        let mut normals = vec![0.0f32; vcount * 3];
        for tri in indices.chunks(3) {
            if tri.len() < 3 { continue; }
            let a = tri[0] as usize * 3;
            let b = tri[1] as usize * 3;
            let c = tri[2] as usize * 3;
            if a + 2 >= vertices.len() || b + 2 >= vertices.len() || c + 2 >= vertices.len() { continue; }
            let (ax, ay, az) = (vertices[a], vertices[a + 1], vertices[a + 2]);
            let (bx, by, bz) = (vertices[b], vertices[b + 1], vertices[b + 2]);
            let (cx, cy, cz) = (vertices[c], vertices[c + 1], vertices[c + 2]);
            let (u1, u2, u3) = (bx - ax, by - ay, bz - az);
            let (v1, v2, v3) = (cx - ax, cy - ay, cz - az);
            let (nx, ny, nz) = (u2 * v3 - u3 * v2, u3 * v1 - u1 * v3, u1 * v2 - u2 * v1);
            for vi in [tri[0] as usize, tri[1] as usize, tri[2] as usize] {
                normals[vi * 3] += nx;
                normals[vi * 3 + 1] += ny;
                normals[vi * 3 + 2] += nz;
            }
        }
        for i in 0..vcount {
            let (x, y, z) = (normals[i * 3], normals[i * 3 + 1], normals[i * 3 + 2]);
            let len = (x * x + y * y + z * z).sqrt();
            if len > 1e-6 {
                normals[i * 3] = x / len;
                normals[i * 3 + 1] = y / len;
                normals[i * 3 + 2] = z / len;
            } else {
                normals[i * 3 + 2] = 1.0;
            }
        }
        normals
    }

    // Добавить UV-координаты для каждой вершины (генерирует плоские дефолты, если нет)
    pub fn with_cube_uvs(mut self) -> Self {
        let n = self.vertices.len() / 3;
        // Кубик: каждой вершине сопоставляем грань. Простая схема: y→u, z→v (по углу от половины)
        self.uvs = Vec::with_capacity(n * 2);
        // Приблизительный UV: (x,y,z) → (0.5 + 0.5*atan2(z,x)/π, 0.5 - y)
        for v in self.vertices.chunks(3) {
            let u = 0.5 + (v[2].atan2(v[0])) / std::f32::consts::TAU;
            let uv = (0.5 + 0.5 * v[2] / self.vertices.len() as f32 * 0.0, v[1] * 0.5 + 0.5);
            self.uvs.push(u);
            self.uvs.push(uv.1);
        }
        self.has_uv = true;
        self
    }

    pub fn with_uvs(mut self, uvs: Vec<f32>) -> Self {
        self.uvs = uvs;
        self.has_uv = !self.uvs.is_empty();
        self
    }
}

impl Default for Mesh {
    fn default() -> Self {
        Self::new()
    }
}