use crate::alglin::*;
const PI: f32 = 3.141592;

fn coordenada_cilindro(angulo: f32, altura: f32, raio: f32) -> [f32; 3] {
    let x = raio * angulo.cos();
    let y = raio * angulo.sin();
    let z = altura;
    [x, y, z]
}

fn coordenada_esfera(angulo_longitude: f32, angulo_latitude: f32, raio: f32) -> Vertex {
    let x = raio * angulo_latitude.sin() * angulo_longitude.cos();
    let y = raio * angulo_latitude.sin() * angulo_longitude.sin();
    let z = raio * angulo_latitude.cos();
    [x, y, z]
}

//Créditos: Prof. Jean Roberto Ponciano
pub fn cria_prisma(radius: f32, height: f32, base: f32) -> Vertices {
    let mut vertices: Vec<[f32; 3]> = Vec::new();
    let sector_count: f32 = base;
    let sector_step: f32 = 2.0 * PI / sector_count;
    let stack_count: f32 = 40.0;
    let stack_step: f32 = height / stack_count;

    for j in 0..(stack_count as i32) {
        for i in 0..(sector_count as i32) {
            let current_sector = (i as f32) * sector_step;
            let current_stack = (j as f32) * stack_step;
            let next_sector = {
                if ((i as f32) + 1.0) == sector_count {
                    PI * 2.0
                } else {
                    ((i as f32) + 1.0) * sector_step
                }
            };

            let next_stack = {
                if ((j as f32) + 1.0) == stack_count {
                    height
                } else {
                    ((j as f32) + 1.0) * stack_step
                }
            };

            let p0 = coordenada_cilindro(current_sector, current_stack, radius);
            let p1 = coordenada_cilindro(current_sector, next_stack, radius);
            let p2 = coordenada_cilindro(next_sector, current_stack, radius);
            let p3 = coordenada_cilindro(next_sector, next_stack, radius);

            vertices.push(p0);
            vertices.push(p2);
            vertices.push(p1);

            vertices.push(p3);
            vertices.push(p1);
            vertices.push(p2);

            if current_stack == 0.0 {
                vertices.push(p0);
                vertices.push(p2);
                vertices.push(coordenada_cilindro(0.0, current_stack, 0.0));
            }
            if next_stack == height {
                vertices.push(p1);
                vertices.push(p3);
                vertices.push(coordenada_cilindro(0.0, next_stack, 0.0));
            }
        }
    }
    vertices
}

pub fn cria_banco(altura: f32, largura: f32) -> Vertices {
    let mut assento = cria_prisma(largura, altura / 12.0, 4.0);
    assento = assento.matrix4fv_mul_vertex(&matriz_rotacao_y(90.0));
    assento = assento.centraliza();
    assento = assento.matrix4fv_mul_vertex(&matriz_rotacao_z(-90.0));
    let mut pe_do_assento = cria_prisma(largura / 6.0, altura / 3.0, 4.0);
    pe_do_assento = pe_do_assento.matrix4fv_mul_vertex(&matriz_rotacao_x(90.0));
    pe_do_assento = pe_do_assento.centraliza();
    pe_do_assento =
        pe_do_assento.matrix4fv_mul_vertex(&matriz_translacao(-largura / 2.0, -0.15, 0.0));
    let mut outro_pe = cria_prisma(largura / 6.0, altura / 3.0, 4.0);
    outro_pe = outro_pe.matrix4fv_mul_vertex(&matriz_rotacao_x(90.0));
    outro_pe = outro_pe.centraliza();
    outro_pe = outro_pe.matrix4fv_mul_vertex(&matriz_translacao(largura / 2.0, -0.15, 0.0));
    assento.append(&mut pe_do_assento);
    assento.append(&mut outro_pe);
    assento.centraliza()
}

pub fn cria_pesos_do_pulldown(raio_peso: f32, height: f32) -> (usize, Vertices) {
    let mut pesos_de_cima = cria_pesos_do_pulldown_privado(raio_peso, height)
        .matrix4fv_mul_vertex(&matriz_rotacao_z(180.0));
    let pesos_de_cima_len = pesos_de_cima.len();
    let mut pesos_de_baixo = cria_pesos_do_pulldown_privado(raio_peso, height);
    pesos_de_cima.append(&mut pesos_de_baixo);
    (pesos_de_cima_len, pesos_de_cima.centraliza())
}

fn cria_pesos_do_pulldown_privado(raio_peso: f32, height: f32) -> Vertices {
    let mut vertices: Vec<[f32; 3]> = Vec::new();
    let sector_count: f32 = 4.0;
    let sector_step: f32 = 2.0 * PI / sector_count;
    let stack_count: f32 = 40.0;
    let stack_step: f32 = height / stack_count;
    let raio_barra = raio_peso / 20.0;
    for j in 0..(stack_count as i32) {
        for i in 0..(sector_count as i32) {
            let current_sector = (i as f32) * sector_step;
            let current_stack = (j as f32) * stack_step;
            let next_sector = {
                if ((i as f32) + 1.0) == sector_count {
                    PI * 2.0
                } else {
                    ((i as f32) + 1.0) * sector_step
                }
            };

            let next_stack = {
                if ((j as f32) + 1.0) == stack_count {
                    height
                } else {
                    ((j as f32) + 1.0) * stack_step
                }
            };
            let p0: [f32; 3];
            let p1: [f32; 3];
            let p2: [f32; 3];
            let p3: [f32; 3];
            if j % 4 != 0 {
                p0 = coordenada_cilindro(current_sector, current_stack, raio_peso);
                p1 = coordenada_cilindro(current_sector, next_stack, raio_peso);
                p2 = coordenada_cilindro(next_sector, current_stack, raio_peso);
                p3 = coordenada_cilindro(next_sector, next_stack, raio_peso);
            } else {
                p0 = coordenada_cilindro(current_sector, current_stack, raio_barra);
                p1 = coordenada_cilindro(current_sector, next_stack, raio_barra);
                p2 = coordenada_cilindro(next_sector, current_stack, raio_barra);
                p3 = coordenada_cilindro(next_sector, next_stack, raio_barra);
            }
            vertices.push(p0);
            vertices.push(p2);
            vertices.push(p1);

            vertices.push(p3);
            vertices.push(p1);
            vertices.push(p2);

            if current_stack == 0.0 {
                vertices.push(p0);
                vertices.push(p2);
                vertices.push(coordenada_cilindro(0.0, current_stack, 0.0));
            }
            if next_stack == height {
                vertices.push(p1);
                vertices.push(p3);
                vertices.push(coordenada_cilindro(0.0, next_stack, 0.0));
            }
        }
    }
    vertices
        .matrix4fv_mul_vertex(&matriz_rotacao_z(45.0))
        .matrix4fv_mul_vertex(&matriz_rotacao_x(90.0))
}

pub fn cria_pulldown(comprimento: f32, raio: f32) -> Vertices {
    let mut centro = cria_prisma(raio, comprimento * 4.0 / 6.0, 40.0);
    let mut esquerda = cria_prisma(raio, comprimento / 6.0, 40.0);
    let mut direita = cria_prisma(raio, comprimento / 6.0, 40.0);
    centro = centro.matrix4fv_mul_vertex(&matriz_rotacao_y(90.0));
    centro = centro.centraliza();
    esquerda = esquerda.matrix4fv_mul_vertex(&matriz_rotacao_y(90.0));
    esquerda = esquerda.centraliza();
    esquerda = esquerda.matrix4fv_mul_vertex(&matriz_rotacao_z(45.0));
    esquerda = esquerda.matrix4fv_mul_vertex(&matriz_translacao(-comprimento / 3.0, -raio, 0.0));
    centro.append(&mut esquerda);
    direita = direita.matrix4fv_mul_vertex(&matriz_rotacao_y(90.0));
    direita = direita.centraliza();
    direita = direita.matrix4fv_mul_vertex(&matriz_rotacao_z(-45.0));
    direita = direita.matrix4fv_mul_vertex(&matriz_translacao(comprimento / 3.0, -raio, 0.0));
    centro.append(&mut direita);
    centro.centraliza()
}

pub fn cria_tronco_e_cabeça(raio: f32, altura: f32) -> Vertices {
    let mut tronco = cria_prisma(raio, altura, 40.0);
    tronco = tronco.matrix4fv_mul_vertex(&matriz_rotacao_y(90.0));
    tronco = tronco.centraliza();
    let mut cabeça = cria_esfera(raio * 3.5 / 5.0);
    cabeça = cabeça.centraliza();
    cabeça = cabeça.matrix4fv_mul_vertex(&matriz_translacao(0.0, altura, 0.0));
    tronco.append(&mut cabeça);

    tronco
}

pub fn cria_antebraços(raio: f32, altura: f32, angulo: f32) -> Vertices {
    let mut antebraço_direito = cria_tronco_e_cabeça(raio, altura)
        .matrix4fv_mul_vertex(&matriz_rotacao_z(angulo))
        .centraliza()
        .matrix4fv_mul_vertex(&matriz_translacao(raio * 1.25, altura * 4.0, 0.0));
    let mut antebraço_esquerdo = cria_tronco_e_cabeça(raio, altura)
        .matrix4fv_mul_vertex(&matriz_rotacao_z(-angulo))
        .centraliza()
        .matrix4fv_mul_vertex(&matriz_translacao(-raio * 1.25, altura * 4.0, 0.0));
    antebraço_esquerdo.append(&mut antebraço_direito);
    antebraço_esquerdo
}
pub fn cria_braços(raio: f32, altura: f32, angulo: f32) -> Vertices {
    let mut braço_esquerdo = cria_prisma(raio, altura, 4.0)
        .matrix4fv_mul_vertex(&matriz_rotacao_y(90.0))
        .centraliza()
        .matrix4fv_mul_vertex(&matriz_rotacao_z(-angulo))
        .centraliza()
        .matrix4fv_mul_vertex(&matriz_translacao(raio * 1.5, altura * 2.0, 0.0));
    let mut braço_direito = cria_prisma(raio, altura, 4.0)
        .matrix4fv_mul_vertex(&matriz_rotacao_y(90.0))
        .centraliza()
        .matrix4fv_mul_vertex(&matriz_rotacao_z(angulo))
        .centraliza()
        .matrix4fv_mul_vertex(&matriz_translacao(-raio * 1.5, altura * 2.0, 0.0));
    braço_esquerdo.append(&mut braço_direito);
    braço_esquerdo
}
pub fn cria_pessoa(raio: f32, altura: f32) -> (usize, usize, usize, Vertices) {
    let mut tronco_e_cabeça = cria_tronco_e_cabeça(raio, altura);
    let mut antebraços = cria_antebraços(raio / 2.0, altura / 2.0, 20.0);
    let mut braços = cria_braços(raio / 2.0, altura / 2.0, 20.0);
    let braços_size = braços.len();
    let antebraço_size = antebraços.len();
    antebraços.append(&mut braços);
    let tronco_e_cabeca_size = tronco_e_cabeça.len();
    antebraços.append(&mut tronco_e_cabeça);
    (
        tronco_e_cabeca_size,
        antebraço_size,
        braços_size,
        antebraços.centraliza(),
    )
}

pub fn cria_pessoa2(comprimento: f32, altura: f32) -> (Vertex,usize,Vertices) {
    let mut tronco: Vertices = Vec::new();
    let v1_tronco:Vertex = [-comprimento / 2.0, altura / 2.0, 0.0];
    let v2_tronco: Vertex = [-comprimento / 2.0, -altura / 2.0, 0.0];
    let v3_tronco: Vertex = [comprimento / 2.0, altura / 2.0, 0.0];
    let v4_tronco: Vertex = [comprimento / 2.0, -altura / 2.0, 0.0];

    tronco.push(v2_tronco);
    tronco.push(v3_tronco);
    tronco.push(v1_tronco);

    tronco.push(v3_tronco);
    tronco.push(v4_tronco);
    tronco.push(v2_tronco);

    let mut cabeca: Vertices = Vec::new();
    let v1_cabeca:Vertex = [-(comprimento + 0.1) / 2.0, comprimento / 2.0, 0.0];
    let v2_cabeca: Vertex = [-(comprimento+0.1) / 2.0, -comprimento / 2.0, 0.0];
    let v3_cabeca: Vertex = [(comprimento+0.1) / 2.0, comprimento / 2.0, 0.0];
    let v4_cabeca: Vertex = [(comprimento+0.1) / 2.0, -comprimento / 2.0, 0.0];

    cabeca.push(v2_cabeca);
    cabeca.push(v3_cabeca);
    cabeca.push(v1_cabeca);

    cabeca.push(v3_cabeca);
    cabeca.push(v4_cabeca);
    cabeca.push(v2_cabeca);
    
    cabeca = cabeca.matrix4fv_mul_vertex(&matriz_translacao(0.0,altura/2.0,0.0));

    let mut braco: Vertices = Vec::new();
    let v1_braco:Vertex = [-comprimento  / 6.0, altura/3.0, 0.0];
    let v2_braco: Vertex = [-comprimento / 6.0, -altura/3.0, 0.0];
    let v3_braco: Vertex = [comprimento / 6.0, altura/3.0, 0.0];
    let v4_braco: Vertex = [comprimento / 6.0, -altura/3.0, 0.0];

    braco.push(v2_braco);
    braco.push(v3_braco);
    braco.push(v1_braco);

    braco.push(v3_braco);
    braco.push(v4_braco);
    braco.push(v2_braco);
    
    braco = braco.matrix4fv_mul_vertex(&matriz_rotacao_z(45.0)).matrix4fv_mul_vertex(&matriz_translacao(comprimento/4.0,altura/8.0,0.0));

    let mut antebraco: Vertices = Vec::new();
    let v1_antebraco:Vertex = [-comprimento  / 6.0, altura/4.0, 0.0];
    let v2_antebraco: Vertex = [-comprimento / 6.0, -altura/4.0, 0.0];
    let v3_antebraco: Vertex = [comprimento / 6.0, altura/4.0, 0.0];
    let v4_antebraco: Vertex = [comprimento / 6.0, -altura/4.0, 0.0];

    antebraco.push(v2_antebraco);
    antebraco.push(v3_antebraco);
    antebraco.push(v1_antebraco);

    antebraco.push(v3_antebraco);
    antebraco.push(v4_antebraco);
    antebraco.push(v2_antebraco);
    let tamanho_antebraco = antebraco.len();
    let centroide_antebraco = calcula_centroide(&antebraco);
    
    antebraco = antebraco.matrix4fv_mul_vertex(&matriz_rotacao_z(-90.0)).matrix4fv_mul_vertex(&matriz_translacao(comprimento*1.1,-0.1,0.0));
    antebraco.append(&mut tronco);
    antebraco.append(&mut cabeca);
    antebraco.append(&mut braco);

    (centroide_antebraco,tamanho_antebraco,antebraco)
}

pub fn cria_halter(raio_barra: f32, raio_peso: f32, height: f32) ->  Vertices {
    let mut vertices: Vec<[f32; 3]> = Vec::new();
    let sector_count: f32 = 40.0;
    let sector_step: f32 = 2.0 * PI / sector_count;
    let stack_count: f32 = 40.0;
    let stack_step: f32 = height / stack_count;

    for j in 0..(stack_count as i32) {
        for i in 0..(sector_count as i32) {
            let current_sector = (i as f32) * sector_step;
            let current_stack = (j as f32) * stack_step;
            let next_sector = {
                if ((i as f32) + 1.0) == sector_count {
                    PI * 2.0
                } else {
                    ((i as f32) + 1.0) * sector_step
                }
            };

            let next_stack = {
                if ((j as f32) + 1.0) == stack_count {
                    height
                } else {
                    ((j as f32) + 1.0) * stack_step
                }
            };
            let p0: [f32; 3];
            let p1: [f32; 3];
            let p2: [f32; 3];
            let p3: [f32; 3];
            if ((j as f32) * stack_step) <= (height / 5.0)
                || ((j as f32) * stack_step) >= (height * 4.0 / 5.0)
            {
                p0 = coordenada_cilindro(current_sector, current_stack, raio_peso);
                p1 = coordenada_cilindro(current_sector, next_stack, raio_peso);
                p2 = coordenada_cilindro(next_sector, current_stack, raio_peso);
                p3 = coordenada_cilindro(next_sector, next_stack, raio_peso);
            } else {
                p0 = coordenada_cilindro(current_sector, current_stack, raio_barra);
                p1 = coordenada_cilindro(current_sector, next_stack, raio_barra);
                p2 = coordenada_cilindro(next_sector, current_stack, raio_barra);
                p3 = coordenada_cilindro(next_sector, next_stack, raio_barra);
            }
            vertices.push(p0);
            vertices.push(p2);
            vertices.push(p1);

            vertices.push(p3);
            vertices.push(p1);
            vertices.push(p2);

            if current_stack == 0.0 {
                vertices.push(p0);
                vertices.push(p2);
                vertices.push(coordenada_cilindro(0.0, current_stack, 0.0));
            }
            if next_stack == height {
                vertices.push(p1);
                vertices.push(p3);
                vertices.push(coordenada_cilindro(0.0, next_stack, 0.0));
            }
        }
    }
       vertices.centraliza()
}

// Creditos: Prof. Jean Roberto Ponciano
pub fn cria_esfera(raio: f32) -> Vertices {
    let sectors: f32 = 40.0;
    let stacks: f32 = 40.0;
    let sector_step: f32 = 2.0 * PI / sectors;
    let stack_step: f32 = PI / stacks;

    let mut vertices: Vec<Vertex> = Vec::new();

    for i in 0..=(sectors as i32) {
        for j in 0..=(stacks as i32) {
            let angulo_setor = (i as f32) * sector_step;
            let angulo_stack = (j as f32) * stack_step;
            let mut angulo_proximo_setor: f32 = 0.0;
            if (i as f32) + 1.0 == sectors {
                angulo_proximo_setor = PI * 2.0;
            } else {
                angulo_proximo_setor = ((i as f32) + 1.0) * sector_step;
            }
            let mut angulo_proxima_stack = 0.0;
            if (angulo_proxima_stack as f32) + 1.0 == stacks {
                angulo_proxima_stack = PI;
            } else {
                angulo_proxima_stack = ((j as f32) + 1.0) * stack_step;
            }
            let p0 = coordenada_esfera(angulo_setor, angulo_stack, raio);
            let p1 = coordenada_esfera(angulo_setor, angulo_proxima_stack, raio);
            let p2 = coordenada_esfera(angulo_proximo_setor, angulo_stack, raio);
            let p3 = coordenada_esfera(angulo_proximo_setor, angulo_proxima_stack, raio);

            vertices.push(p0);
            vertices.push(p2);
            vertices.push(p1);

            vertices.push(p3);
            vertices.push(p1);
            vertices.push(p2);
        }
    }
    vertices
}
