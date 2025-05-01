use std::{fs::File, io::Read};

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
    File::open(arquivo)
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
