extern crate glfw;
use glfw::{Action, Context, Key};
use gl33::*;
use std::ffi::{c_float,CString};
mod cilindro;
use cilindro::cria_cilindro;
fn main() {
    use glfw::fail_on_errors;
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();

    // Cria uma janela glfw
    let (mut window, events) = glfw
        .create_window(800,600,"Uma janela",glfw::WindowMode::Windowed)
        .expect("Falha em criar uma janela glfw");
    window.make_current();
    window.set_key_polling(true);
    // Dado um contexto (vindo do glfw) e um processo, do opengl, obtém o endereço do processo e
    // carrega o opengl com esse endereço
    let gl = unsafe{
        GlFns::load_from(&|p|{
                glfw.get_proc_address_raw(&glfw::string_from_c_str(p as *const _)) as *const _
        })
        .unwrap()
    };
   
    // define o vertex array object
    unsafe {
        let mut vao = 0;
        gl.GenVertexArrays(1, &mut vao);
        //o vertex array object não pode ser zero depois dessa operação
        assert_ne!(vao,0);

    //define o vertex buffer object

        let mut vbo = 0;
        gl.GenBuffers(1, &mut vbo);
        assert_ne!(vbo,0);
    // Vincula o vbo
        gl.BindBuffer(GL_ARRAY_BUFFER,vbo);

    // Vertices:
    let vertices_cilindro = cria_cilindro(0.1,0.9);
    let n_vertices_cilindro = vertices_cilindro.len();
    // Envia nossos vértices pro array buffer
        gl.BufferData(GL_ARRAY_BUFFER,
            size_of_val(&vertices_cilindro) as isize,
            vertices_cilindro.as_ptr().cast(),
            GL_DYNAMIC_DRAW,
        );

    //Criação do vertex shader program object
        let vertex_shader = gl.CreateShader(GL_VERTEX_SHADER);
        assert_ne!(vertex_shader,0);
        const VERT_SHADER: &str = r#"#version 330 core
        attribute vec3 position;
        uniform mat4 mat_transformation;
        void main() {
        gl_Position = vec4(position, 1.0);
        }"#;
    // envia o código fonte do shader para o nosso vspo
    
        gl.ShaderSource(
            vertex_shader,
            1, // numero de elementos dos vetores string e tamanho de string. Eu não sei o porquê
               // disso, mas está na documentação https://registry.khronos.org/OpenGL-Refpages/gl4/html/glShaderSource.xhtml
            &(VERT_SHADER.as_bytes().as_ptr().cast()), // a string do código fonte
            &(VERT_SHADER.len().try_into().unwrap()), // o tamanho da string
        );
    // Compila o vertex shader
        gl.CompileShader(vertex_shader);
    
    // Verifica se a compilação deu certo
    
        let mut success = 0;
        gl.GetShaderiv(vertex_shader,GL_COMPILE_STATUS,&mut success);

        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024); //Vetor que armazena o log de erro
            let mut log_len = 0_i32;
            gl.GetShaderInfoLog(
                vertex_shader,
                1024,
                &mut log_len,
                v.as_mut_ptr().cast(),
            );
            v.set_len(log_len.try_into().unwrap());
            panic!("Erro na compilação do vertex shader: {:?}", String::from_utf8_lossy(&v));
       }

    // Fragment Shader

        let fragment_shader = gl.CreateShader(GL_FRAGMENT_SHADER);
        assert_ne!(fragment_shader,0);

    const FRAG_SHADER: &str = r#"#version 330 core
    out vec4 final_color;
    uniform vec4 color;
    void main(){
        gl_FragColor = color;
    }
        "#;

    // Compila o shader e verifica se a compilação deu certo
        gl.ShaderSource(
            fragment_shader,
            1,
            &(FRAG_SHADER.as_bytes().as_ptr().cast()),
            &(FRAG_SHADER.len().try_into().unwrap()),
        );
        gl.CompileShader(fragment_shader);

        let mut success = 0;
        gl.GetShaderiv(fragment_shader, GL_COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024); //Vetor que armazena o log de erro
            let mut log_len = 0_i32;
            gl.GetShaderInfoLog(
                fragment_shader,
                1024,
                &mut log_len,
                v.as_mut_ptr().cast(),
            );
            v.set_len(log_len.try_into().unwrap());
            panic!("Erro na compilação do fragment shader: {:?}", String::from_utf8_lossy(&v));
        }
    // Cria um programa utilizando nossos shaders
        let shader_program = gl.CreateProgram();
        gl.AttachShader(shader_program, vertex_shader);
        gl.AttachShader(shader_program, fragment_shader);
        gl.LinkProgram(shader_program);
        
        let mut success = 0;
        gl.GetProgramiv(shader_program, GL_LINK_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl.GetProgramInfoLog(
                shader_program,
                1024,
                &mut log_len,
                v.as_mut_ptr().cast(),
            );
            v.set_len(log_len.try_into().unwrap());
            panic!("Erro da linkagem dos shaders no programa: {}", String::from_utf8_lossy(&v));
        }

            // Dados pro vertex attribute pointer
        let loc = gl.GetAttribLocation(shader_program,CString::new("position").unwrap().as_ptr() as *const u8);
        gl.EnableVertexAttribArray(loc.try_into().unwrap());
        gl.VertexAttribPointer(loc.try_into().unwrap(),3,GL_FLOAT,0,size_of::<Vertex>().try_into().unwrap(),std::ptr::null(),);

    // Agora que os shaders foram linkados, podemos deletar eles
        gl.DeleteShader(vertex_shader);
        gl.DeleteShader(fragment_shader);
    // Vsync
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1_u32));

    // position

    
    // Muda a cor do fundo
    gl.ClearColor(0.3,0.3,0.3,1.0);
    
    //loop até usuário fechar a janela
    while !window.should_close() {
            gl.Clear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
            let loc_color = gl.GetUniformLocation(shader_program,CString::new("color").unwrap().as_ptr() as *const u8);
            let loc = gl.GetUniformLocation(shader_program,CString::new("position").unwrap().as_ptr() as *const u8);
            gl.UniformMatrix4fv(loc,1,1,IDENTITY_MATRIX.to_matrix4fv());
            for triangle in (0..vertices_cilindro.len()).step_by(3) {
                gl.Uniform4f(loc_color.try_into().unwrap(),0.5,0.5,1.0,1.0);
                gl.DrawArrays(GL_TRIANGLES,triangle.try_into().unwrap(),3);
            }

        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}",event);
            if let glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
                window.set_should_close(true)
            }
        }
        window.swap_buffers();
        glfw.poll_events();

    }
    }
}
type Vertex = [f32;3];
type V4Matrix = [[f32;4];4];
const IDENTITY_MATRIX:V4Matrix = [[1.0,0.0,0.0,0.0], [0.0,1.0,0.0,0.0],[0.0,0.0,1.0,0.0],[0.0,0.0,0.0,1.0]];
pub trait Matrix4fvTransformable {
    fn to_matrix4fv(&self) -> *const c_float;
}

impl Matrix4fvTransformable for V4Matrix {
    fn to_matrix4fv(&self) -> *const c_float {
        self.as_ptr() as *const c_float
    }
}
