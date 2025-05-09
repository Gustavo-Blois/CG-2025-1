use std::{fs::File, io::Read,env,path::PathBuf};
use image::*;
use gl33::*;
use gl33::global_loader::*;
#[derive(Debug)]
pub struct Objeto {
    pub vertices: Vec<[f32; 3]>,
    pub texturas: Vec<[f32; 2]>,
    pub material: String,
    pub faces: Vec<[[u32; 3]; 3]>,
}

pub fn cria_objeto(arquivo: String) -> Objeto {
    let mut dados = String::new();
    let mut vertices: Vec<[f32; 3]> = Vec::new();
    let mut texturas: Vec<[f32; 2]> = Vec::new();
    let mut faces: Vec<[[u32; 3]; 3]> = Vec::new();
    let mut material: String = String::from("");
    // Cargo manifest dir é o diretório que contém o cargo.toml
    // essa variável de ambiente nos permite capturar arquivos
    // assets de forma agnóstica à onde o programa foi chamado
    let mut objects = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("Não foi possível capturar diretório"));
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
                let c1: Vec<&str> = v[1].split('/').collect();
                let c2: Vec<&str> = v[2].split('/').collect();
                let c3: Vec<&str> = v[3].split('/').collect();
                faces.push([
                    [
                        c1[0].parse::<u32>().unwrap(),
                        c1[1].parse::<u32>().unwrap(),
                        c1[2].parse::<u32>().unwrap(),
                    ],
                    [
                        c2[0].parse::<u32>().unwrap(),
                        c2[1].parse::<u32>().unwrap(),
                        c2[2].parse::<u32>().unwrap(),
                    ],
                    [
                        c3[0].parse::<u32>().unwrap(),
                        c3[1].parse::<u32>().unwrap(),
                        c3[2].parse::<u32>().unwrap(),
                    ],
                ]);
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

fn load_obj_and_texture(objeto:&str, texturas:Vec<String>, lista_de_vertices:&mut Vec<[f32;3]>, lista_de_texturas: &mut Vec<[f32;2]>) -> (usize,usize) {
    let modelo = cria_objeto(objeto.to_string());
    let vertice_inicial = lista_de_vertices.len();
    println!("Processando {}. Vertice inicial: {}",objeto, vertice_inicial);
    let mut faces_visitadas = Vec::new();

    for face in modelo.faces.iter() {
        if !faces_visitadas.contains(&face[2]) {
            faces_visitadas.push(face[2].clone());
        }
        for vertice_id in circular_sliding_window_of_three(face[0].to_vec()){
            lista_de_vertices.push(modelo.vertices[(vertice_id -1) as usize]).clone();
        }
        for texture_id in circular_sliding_window_of_three(face[1].to_vec()){
            lista_de_texturas.push(modelo.texturas[(texture_id-1) as usize]).clone();
        }
    }
    let vertice_final = lista_de_vertices.len();
    println!("Processando modelo {}. Vertice final {}",objeto,vertice_final); 

    let mut numero_texturas:usize = 0;
    for textura in texturas.iter() {
        load_texture_from_file(numero_texturas,textura.to_string());
        numero_texturas +=1;
    }
    return (vertice_inicial, vertice_final - vertice_inicial);
}

// é esperado que img_textura tenha formato nome_modelo/nome_modelo.extensão
fn load_texture_from_file(texture_id: usize, img_textura: String) {
    let mut textura = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("Não foi possível capturar diretório"));
    textura.push("objetos");
    textura.push(&img_textura);
    println!("{}",texture_id);
    unsafe {glBindTexture(GL_TEXTURE_2D, texture_id.try_into().unwrap());
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_REPEAT);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR);
    let img = ImageReader::open(textura).expect("falha em abrir textura").decode().expect("falha em decodificar textura");
    let (height, width) = img.dimensions();
    let image_data = img.into_bytes();
    glTexImage2D(GL_TEXTURE_2D, 0, GL_RGB, width.try_into().unwrap(), height.try_into().unwrap(), 0, GL_RGB, GL_UNSIGNED_BYTE, image_data.as_ptr().cast());
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
    for i in 0..(circular_arr.len() -2) {
        result.extend_from_slice(&circular_arr[i..i+3])
    }
    return result;
}



#[cfg(test)]
mod tests {
    use super::*;
    #[ignore]
    fn test_object(){
        
        let miranha = cria_objeto(
            "spiderman"
                .to_string(),
        );
        panic!("{:?}",miranha);
    }
    #[test]
    fn test_circular_sliding_window() {
        let vector = vec![1,2,3,4,5,6,7,8,9,10,11];
        panic!("{:?}",circular_sliding_window_of_three(vector));
    }
}
