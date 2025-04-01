const PI: f32 = 3.141592;

fn coordenada_cilindro(angulo:f32,altura:f32,raio:f32) -> [f32;3] {
    let x = raio*angulo.cos();
    let y = raio*angulo.sin();
    let z = altura;
    [x,y,z]
}

//Por padrão, os cilindros serão prismas de base 40
pub fn cria_cilindro(radius:f32,height:f32) -> Vec<[f32;3]> {
    let mut vertices: Vec<[f32;3]> = Vec::new();
    let sector_count:f32 = 40.0;
    let sector_step:f32 = 2.0*PI/sector_count;
    let stack_count:f32 = 40.0; 
    let stack_step:f32 = height/stack_count;

    for j in 0..(stack_count as i32) {
        for i in 0..(sector_count as i32) {
            let current_sector = (i as f32)*sector_step;
            let current_stack = (j as f32)*stack_step;
            let next_sector = {
                if ((i as f32)+1.0) == sector_count{
                PI*2.0
                } else {
                ((i as f32)+1.0)*sector_step
                }
            };

            let next_stack = {
                if ((j as f32)+1.0) == stack_count {
                    height
                } else {
                    ((j as f32)+1.0)*stack_step
                }
            };
            
            let p0 = coordenada_cilindro(current_sector,current_stack,radius);
            let p1 = coordenada_cilindro(current_sector,next_stack,radius);
            let p2 = coordenada_cilindro(next_sector,current_stack,radius);
            let p3 = coordenada_cilindro(next_sector,next_stack,radius);
            
            vertices.push(p0);
            vertices.push(p2);
            vertices.push(p1);

            vertices.push(p3);
            vertices.push(p1);
            vertices.push(p2);

            if current_stack == 0.0{
                vertices.push(p0);
                vertices.push(p2);
                vertices.push(coordenada_cilindro(0.0,current_stack,0.0));
            }
            if next_stack == height {
                vertices.push(p1);
                vertices.push(p3);
                vertices.push(coordenada_cilindro(0.0,next_stack,0.0));
            }
        }
        
    }
    vertices
}
