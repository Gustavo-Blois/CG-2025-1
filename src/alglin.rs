use std::ffi::c_float;
pub type Vertex = [f32; 3];
pub type V4Matrix = [[f32; 4]; 4];
pub const IDENTITY_MATRIX: V4Matrix = [
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
];
pub trait Matrix4fvTransformable {
    fn to_matrix4fv(&self) -> *const c_float;
}

pub trait V4MatrixUtils {
    fn multiplication(&self, another_matrix: &V4Matrix) -> V4Matrix;
}

impl V4MatrixUtils for V4Matrix {
    fn multiplication(&self, another_matrix: &V4Matrix) -> V4Matrix {
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

impl Matrix4fvTransformable for V4Matrix {
    fn to_matrix4fv(&self) -> *const c_float {
        self.as_ptr() as *const c_float
    }
}
