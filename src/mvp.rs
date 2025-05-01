use crate::alglin::*;

struct Camera {
    camera_pos: Vertex,
    camera_front: Vertex,
    camera_up: Vertex,
}

impl Camera {
    fn new() -> Self {
        Self {
            camera_pos: [0.0, 0.0, 3.0],
            camera_front: [0.0, 0.0, -1.0],
            camera_up: [0.0, 1.0, 0.0],
        }
    }
}

fn model(
    angle: f32,
    r_x: f32,
    r_y: f32,
    r_z: f32,
    t_x: f32,
    t_y: f32,
    t_z: f32,
    s_x: f32,
    s_y: f32,
    s_z: f32,
) -> V4Matrix {
    let angle = angle.to_radians();
    let mut matriz_transformacao = IDENTITY_MATRIX;

    matriz_transformacao = matriz_transformacao.multiply(&matriz_translacao(t_x, t_y, t_z));

    matriz_transformacao = matriz_transformacao.multiply(&matriz_rotacao(angle, [r_x, r_y, r_z]));

    matriz_transformacao = matriz_transformacao.multiply(&matriz_escala(s_x, s_y, s_z));

    matriz_transformacao
}

fn view(camera: Camera) -> V4Matrix {
    let [px, py, pz] = camera.camera_pos;
    let matriz_translacao = matriz_translacao(-px, -py, -pz);
    let [lx, ly, lz] = camera.camera_front;
    let [izc, jzc, kzc] = normalize([px - lx, py - ly, pz - lz]);
    let [ixc, jxc, kxc] = normalize(produto_vetorial(camera.camera_up, [izc, jzc, kzc]));
    let [iyc, jyc, kyc] = produto_vetorial([izc, jzc, kzc], [ixc, jxc, kxc]);
    let matriz_rotacao = [
        [ixc, jxc, kxc, 0.0],
        [iyc, jyc, kyc, 0.0],
        [izc, jzc, kzc, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
    let matriz_view = matriz_rotacao.multiply(&matriz_translacao);
    matriz_view
}

fn projection(near: f32, far: f32, top: f32, bottom: f32, right: f32, left: f32) -> V4Matrix {
    [
        [
            2.0 * near / right - left,
            0.0,
            right + left / right - left,
            0.0,
        ],
        [
            0.0,
            2.0 * near / top - bottom,
            top + bottom / top - bottom,
            0.0,
        ],
        [
            0.0,
            0.0,
            (-1.0 * (far + near)) / far - near,
            (-2.0 * far * near) / far - near,
        ],
        [0.0, 0.0, -1.0, 0.0],
    ]
}
