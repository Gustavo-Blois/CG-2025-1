use crate::alglin::V4MatrixUtils;
use crate::mvp::*;
use gl33::global_loader::*;
use gl33::*;
use image::*;
use std::{env, ffi::CString, fs::File, io::Read, path::PathBuf};
#[derive(Debug)]
pub struct Objeto {
    pub vertices: Vec<[f32; 3]>,
    pub texturas: Vec<[f32; 2]>,
    pub material: String,
    pub faces: Vec<(Vec<u32>, Vec<u32>, String)>,
}

pub fn cria_objeto(arquivo: String) -> Objeto {
    let mut dados = String::new();
    let mut vertices: Vec<[f32; 3]> = Vec::new();
    let mut texturas: Vec<[f32; 2]> = Vec::new();
    let mut faces: Vec<(Vec<u32>, Vec<u32>, String)> = Vec::new();
    let mut material: String = String::from("");
    // Cargo manifest dir é o diretório que contém o cargo.toml
    // essa variável de ambiente nos permite capturar arquivos
    // assets de forma agnóstica à onde o programa foi chamado
    let mut objects =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("Não foi possível capturar diretório"));
    objects.push("objetos");
    objects.push(&arquivo);
    objects.push(&arquivo);
    objects.set_extension("obj");
    File::open(objects)
        .unwrap()
        .read_to_string(&mut dados)
        .unwrap();

    // para cada linha de dados
    for line in dados.lines() {
        //para cada termo separado por espaço na linha
        let v: Vec<&str> = line.split(' ').collect();
        match v[0] {
            // Se for um vértice, converte os 3 valores de str para f32, gera um vetor com
            // esses valores e insere esse vetor
            "v" => vertices.push([
                v[1].parse::<f32>().unwrap(),
                v[2].parse::<f32>().unwrap(),
                v[3].parse::<f32>().unwrap(),
            ]),
            // Se for uma coordenada de textura, faz algo semelhante
            "vt" => texturas.push([v[1].parse::<f32>().unwrap(), v[2].parse::<f32>().unwrap()]),
            // Se for uma face, gera um vetor de vetores de 3 indices, correspondentes à vértices
            "f" => {
                let mut face: Vec<u32> = Vec::new();
                let mut face_texture: Vec<u32> = Vec::new();
                for value in &v[1..] {
                    let w: Vec<&str> = value.split('/').collect();
                    face.push(w[0].parse::<u32>().unwrap());
                    if w.len() >= 2 && w[1].len() > 0 {
                        face_texture.push(w[1].parse::<u32>().unwrap())
                    } else {
                        face_texture.push(0)
                    }
                }
                faces.push((face, face_texture, material.clone()));
            }
            "usemtl" | "usemat" => {
                material = v[1].to_string();
            }
            _ => {}
        }
    }

    Objeto {
        vertices,
        texturas,
        material,
        faces,
    }
}

pub fn load_obj_and_texture(
    objeto: &str,
    texturas: Vec<String>,
    lista_de_vertices: &mut Vec<[f32; 3]>,
    lista_de_texturas: &mut Vec<[f32; 2]>,
) -> (usize, usize) {
    let modelo = cria_objeto(objeto.to_string());
    let vertice_inicial = lista_de_vertices.len();
    println!(
        "Processando {}. Vertice inicial: {}",
        objeto, vertice_inicial
    );
    println!(
        "o objeto {} tem {} faces\n{} vertices\n{} texturas\n {}material",
        objeto,
        modelo.faces.len(),
        modelo.vertices.len(),
        modelo.texturas.len(),
        modelo.material.len()
    );
    let mut faces_visitadas = Vec::new();

    for (vertices, texturas, material) in modelo.faces.iter() {
        if !faces_visitadas.contains(&material) {
            faces_visitadas.push(material);
        }
        for vertice_id in circular_sliding_window_of_three(vertices.to_vec()) {
            lista_de_vertices.push(modelo.vertices[(vertice_id - 1) as usize]);
        }
        for texture_id in circular_sliding_window_of_three(texturas.to_vec()) {
            lista_de_texturas.push(modelo.texturas[(texture_id - 1) as usize]);
        }
    }
    let vertice_final = lista_de_vertices.len();
    println!(
        "Processando modelo {}. Vertice final {}",
        objeto, vertice_final
    );

    let mut numero_texturas: usize = 0;
    for textura in texturas.iter() {
        load_texture_from_file(numero_texturas, textura.to_string());
        numero_texturas += 1;
    }
    return (vertice_inicial, vertice_final - vertice_inicial);
}

// é esperado que img_textura tenha formato nome_modelo/nome_modelo.extensão
fn load_texture_from_file(texture_id: usize, img_textura: String) {
    let mut textura =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("Não foi possível capturar diretório"));
    textura.push("objetos");
    textura.push(&img_textura);
    println!("{}", texture_id);
    unsafe {
        glBindTexture(GL_TEXTURE_2D, texture_id.try_into().unwrap());
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, 0x2901);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, 0x2901); //0x2901 == gl_repeat
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, 0x2601);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, 0x2601); //0x2601 == gl_linear
        let img = ImageReader::open(textura)
            .expect("falha em abrir textura")
            .decode()
            .expect("falha em decodificar textura");
        let (height, width) = img.dimensions();
        let image_data = img.flipv().into_bytes();
        glTexImage2D(
            GL_TEXTURE_2D,
            0,
            0x1907,
            width.try_into().unwrap(),
            height.try_into().unwrap(),
            0,
            GL_RGB,
            GL_UNSIGNED_BYTE,
            image_data.as_ptr().cast(),
        );
    }
}

fn circular_sliding_window_of_three<T: std::clone::Clone>(vector: Vec<T>) -> Vec<T> {
    if vector.len() == 3 {
        return vector;
    }
    let mut circular_arr = Vec::new();
    circular_arr.extend_from_slice(&vector);
    circular_arr.push(vector[0].clone());
    let mut result = Vec::new();
    for i in 0..(circular_arr.len() - 2) {
        result.extend_from_slice(&circular_arr[i..i + 3])
    }
    return result;
}

pub unsafe fn desenha_objeto(
    vertice_inicial: usize,
    vertice_final: usize,
    program: u32,
    texture_id: usize,
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
) {
    unsafe {
        //matriz model
        let mat_model = model(angle, r_x, r_y, r_z, t_x, t_y, t_z, s_x, s_y, s_z);
        let loc_model = glGetUniformLocation(
            program,
            CString::new("model").unwrap().as_ptr() as *const u8,
        );
        glUniformMatrix4fv(loc_model, 1, 1, mat_model.to_matrix4fv());
        glBindTexture(GL_TEXTURE_2D, texture_id.try_into().unwrap());
        glDrawArrays(
            GL_TRIANGLES,
            vertice_inicial.try_into().unwrap(),
            vertice_final.try_into().unwrap(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[ignore]
    fn test_object() {
        let miranha = cria_objeto("spiderman".to_string());
        panic!("{:?}", miranha);
    }
    #[test]
    fn test_circular_sliding_window() {
        let vector = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        panic!("{:?}", circular_sliding_window_of_three(vector));
    }
}
