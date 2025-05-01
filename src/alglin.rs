use std::ffi::c_float;
pub type Vertex = [f32; 3];
pub type Vertices = Vec<Vertex>;
pub type V4Matrix = [[f32; 4]; 4];
pub const IDENTITY_MATRIX: V4Matrix = [
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
];

pub fn calcula_centroide(vertices: &Vec<Vertex>) -> Vertex {
    let mut acumulador_x = 0.0;
    let mut acumulador_y = 0.0;
    let mut acumulador_z = 0.0;
    let mut n_items = 0.0;
    for vertice in vertices.iter() {
        let [x, y, z] = vertice;
        acumulador_x += x;
        acumulador_y += y;
        acumulador_z += z;

        n_items += 1.0;
    }
    [
        acumulador_x / n_items,
        acumulador_y / n_items,
        acumulador_z / n_items,
    ]
}

pub trait VertexUtils {
    fn matrix4fv_mul_vertex(&self, matrix: &V4Matrix) -> Self;

    fn centraliza(&self) -> Self;
}

impl VertexUtils for Vertex {
    fn centraliza(&self) -> Vertex {
        self.matrix4fv_mul_vertex(&matriz_translacao(-self[0], -self[1], -self[2]))
    }
    fn matrix4fv_mul_vertex(&self, matrix: &V4Matrix) -> Vertex {
        [
            matrix[0][0] * self[0]
                + matrix[0][1] * self[1]
                + matrix[0][2] * self[2]
                + matrix[0][3] * 1.0,
            matrix[1][0] * self[0]
                + matrix[1][1] * self[1]
                + matrix[1][2] * self[2]
                + matrix[1][3] * 1.0,
            matrix[2][0] * self[0]
                + matrix[2][1] * self[1]
                + matrix[2][2] * self[2]
                + matrix[2][3] * 1.0,
        ]
    }
}

impl VertexUtils for Vertices {
    fn centraliza(&self) -> Vertices {
        let [x, y, z] = calcula_centroide(self);
        self.matrix4fv_mul_vertex(&matriz_translacao(-x, -y, -z))
    }
    fn matrix4fv_mul_vertex(&self, matrix: &V4Matrix) -> Vertices {
        let mut novos_vertices: Vertices = Vec::new();
        for vertice in self.iter() {
            novos_vertices.push(vertice.matrix4fv_mul_vertex(&matrix));
        }
        novos_vertices
    }
}

pub trait V4MatrixUtils {
    fn to_matrix4fv(&self) -> *const c_float;
    fn multiply(&self, another_matrix: &V4Matrix) -> V4Matrix;
    #[allow(dead_code)]
    fn transpose(&self) -> V4Matrix;
}

impl V4MatrixUtils for V4Matrix {
    fn multiply(&self, another_matrix: &V4Matrix) -> V4Matrix {
        let mut result: V4Matrix = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                let mut soma: f32 = 0.0;
                for k in 0..4 {
                    soma += self[i][k] * another_matrix[k][j];
                }
                result[i][j] = soma;
            }
        }
        return result;
    }
    fn to_matrix4fv(&self) -> *const c_float {
        self.as_ptr() as *const c_float
    }

    fn transpose(&self) -> V4Matrix {
        let mut new_v4matrix: V4Matrix = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                new_v4matrix[i][j] = self[j][i]
            }
        }
        new_v4matrix
    }
}

pub fn matriz_escala(sx: f32, sy: f32, sz: f32) -> V4Matrix {
    [
        [sx, 0.0, 0.0, 0.0],
        [0.0, sy, 0.0, 0.0],
        [0.0, 0.0, sz, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

pub fn matriz_translacao(tx: f32, ty: f32, tz: f32) -> V4Matrix {
    [
        [1.0, 0.0, 0.0, tx],
        [0.0, 1.0, 0.0, ty],
        [0.0, 0.0, 1.0, tz],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

pub fn produto_vetorial(a: Vertex, b: Vertex) -> Vertex {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

pub fn normalize(vertice: Vertex) -> Vertex {
    let [a, b, c] = vertice;
    let produto_vetorial = a * a + b * b + c * c;
    let inverso_produto_vetorial = 1.0 / (produto_vetorial.sqrt());
    let a2 = a * inverso_produto_vetorial;
    let b2 = b * inverso_produto_vetorial;
    let c2 = c * inverso_produto_vetorial;
    [a2, b2, c2]
}

pub fn matriz_rotacao(angulo: f32, eixo: Vertex) -> V4Matrix {
    let (sin, cos) = angulo.sin_cos();
    let [x, y, z] = normalize(eixo);
    let a = x * (1.0 - cos);
    let b = x * (1.0 - cos);
    let c = x * (1.0 - cos);

    [
        [cos + a * x, a * y + sin * z, a * z - sin * y, 0.0],
        [b * x - sin * z, cos + b * y, b * z + sin * x, 0.0],
        [c * x + sin * y, c * y - sin * x, cos + c * z, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

pub fn matriz_rotacao_x(angulo: f32) -> V4Matrix {
    [
        [1.0, 0.0, 0.0, 0.0],
        [
            0.0,
            angulo.to_radians().cos(),
            -angulo.to_radians().sin(),
            0.0,
        ],
        [
            0.0,
            angulo.to_radians().sin(),
            angulo.to_radians().cos(),
            0.0,
        ],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

pub fn matriz_rotacao_y(angulo: f32) -> V4Matrix {
    [
        [
            angulo.to_radians().cos(),
            0.0,
            -angulo.to_radians().sin(),
            0.0,
        ],
        [0.0, 1.0, 0.0, 0.0],
        [
            angulo.to_radians().sin(),
            0.0,
            angulo.to_radians().cos(),
            0.0,
        ],
        [0.0, 0.0, 0.0, 1.0],
    ]
}
pub fn matriz_rotacao_z(angulo: f32) -> V4Matrix {
    [
        [
            angulo.to_radians().cos(),
            -angulo.to_radians().sin(),
            0.0,
            0.0,
        ],
        [
            angulo.to_radians().sin(),
            angulo.to_radians().cos(),
            0.0,
            0.0,
        ],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}
