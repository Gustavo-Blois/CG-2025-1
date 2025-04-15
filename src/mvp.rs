use crate::alglin::*;

struct Camera {
    camera_pos: Vertex,
    camera_front: Vertex,
    camera_up: Vertex,
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

fn view(camera: Camera) -> V4Matrix {}

fn projection() -> V4Matrix {}
