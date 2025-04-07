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
pub fn cria_prisma(radius: f32, height: f32, base: f32) -> Vec<[f32; 3]> {
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

pub fn cria_banco(altura:f32, largura:f32) -> Vertices {
    let mut assento = cria_prisma(largura,altura/12.0,4.0);
     assento = assento.matrix4fv_mul_vertex(&matriz_rotacao_y(90.0));
    assento = assento.centraliza();
    assento = assento.matrix4fv_mul_vertex(&matriz_rotacao_z(-90.0));
    let mut pe_do_assento = cria_prisma(largura/6.0,altura/3.0,4.0);
    pe_do_assento = pe_do_assento.matrix4fv_mul_vertex(&matriz_rotacao_x(90.0));
    pe_do_assento = pe_do_assento.centraliza();
    pe_do_assento = pe_do_assento.matrix4fv_mul_vertex(&matriz_translacao(-largura/2.0,-0.15,0.0));
    let mut outro_pe = cria_prisma(largura/6.0,altura/3.0,4.0);
    outro_pe = outro_pe.matrix4fv_mul_vertex(&matriz_rotacao_x(90.0));
    outro_pe = outro_pe.centraliza();
    outro_pe = outro_pe.matrix4fv_mul_vertex(&matriz_translacao(largura/2.0,-0.15,0.0));
    assento.append(&mut pe_do_assento);
    assento.append(&mut outro_pe);
    assento
}

pub fn cria_pulldown(comprimento: f32, raio: f32) -> Vertices {
    let mut centro = cria_prisma(raio, comprimento * 4.0 / 6.0, 40.0);
    let mut esquerda = cria_prisma(raio,comprimento/6.0,40.0);
    let mut direita = cria_prisma(raio,comprimento/6.0,40.0);
    centro = centro.matrix4fv_mul_vertex(&matriz_rotacao_y(90.0));
    centro = centro.centraliza();
    esquerda = esquerda.matrix4fv_mul_vertex(&matriz_rotacao_y(90.0));
    esquerda = esquerda.centraliza();
    esquerda = esquerda.matrix4fv_mul_vertex(&matriz_rotacao_z(45.0));
    esquerda = esquerda.matrix4fv_mul_vertex(&matriz_translacao(-comprimento/3.0,-raio,0.0));
    centro.append(&mut esquerda);
    direita = direita.matrix4fv_mul_vertex(&matriz_rotacao_y(90.0));
    direita = direita.centraliza();
    direita = direita.matrix4fv_mul_vertex(&matriz_rotacao_z(-45.0));
    direita = direita.matrix4fv_mul_vertex(&matriz_translacao(comprimento/3.0,-raio,0.0));
    centro.append(&mut direita);
    centro

}

pub fn cria_pessoa(raio:f32, altura:f32) -> Vertices {
    let mut tronco = cria_prisma(raio, altura,4.0);
    tronco = tronco.matrix4fv_mul_vertex(&matriz_rotacao_y(90.0));
    tronco = tronco.centraliza();
    let mut cabeça = cria_esfera(raio*3.5/5.0);
    cabeça = cabeça.centraliza();
    cabeça = cabeça.matrix4fv_mul_vertex(&matriz_translacao(0.0,altura,0.0));
    tronco.append(&mut cabeça);
    tronco
}

pub fn cria_halter(raio_barra: f32, raio_peso: f32, height: f32) -> Vec<[f32; 3]> {
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
    vertices
}


// Creditos: Prof. Jean Roberto Ponciano
pub fn cria_esfera(raio: f32) -> Vec<Vertex> {
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
