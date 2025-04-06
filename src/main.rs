extern crate glfw;
use gl33::*;
use glfw::{Action, Context, Key};
use rand::prelude::*;
use std::ffi::{CString};

mod alglin;
use alglin::*;
mod cilindro;
use cilindro::*;

const VERT_SHADER: &str = r#"#version 330 core
attribute vec3 position;
uniform mat4 mat_transformation;
void main() {
    gl_Position = mat_transformation*vec4(position, 1.0);
}"#;

const FRAG_SHADER: &str = r#"
uniform vec4 color;
void main(){
    gl_FragColor = color;
}"#;

fn main() {
    use glfw::fail_on_errors;
    
    // Inicialização GLFW e janela
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();
    let (mut window, events) = glfw
        .create_window(800, 600, "Uma janela", glfw::WindowMode::Windowed)
        .expect("Falha em criar uma janela glfw");
    
    window.make_current();
    window.set_key_polling(true);

    // Carrega funções OpenGL
    let gl = unsafe {
        GlFns::load_from(&|p| {
            glfw.get_proc_address_raw(&glfw::string_from_c_str(p as *const _)) as *const _
        })
        .unwrap()
    };

    unsafe {
        // Configuração de VAO e VBO
        let mut vao = 0;
        gl.GenVertexArrays(1, &mut vao);
        assert_ne!(vao, 0);

        let mut vbo = 0;
        gl.GenBuffers(1, &mut vbo);
        assert_ne!(vbo, 0);

        // Configuração dos buffers
        gl.BindBuffer(GL_ARRAY_BUFFER, vbo);
        let vertices_cilindro = cria_halter(0.05, 0.2, 0.9);
        let n_vertices_cilindro = vertices_cilindro.len();
        
        gl.BufferData(
            GL_ARRAY_BUFFER,
            (vertices_cilindro.len() * std::mem::size_of::<[f32; 3]>()) as isize,
            vertices_cilindro.as_ptr().cast(),
            GL_DYNAMIC_DRAW,
        );

        // Criação e compilação dos shaders
        let vertex_shader = compile_shader(&gl, GL_VERTEX_SHADER, VERT_SHADER);
        let fragment_shader = compile_shader(&gl, GL_FRAGMENT_SHADER, FRAG_SHADER);

        // Criação do programa de shader
        let shader_program = create_shader_program(&gl, vertex_shader, fragment_shader);
        
        // Configuração dos atributos de vértice
        gl.BindVertexArray(vao);
        gl.UseProgram(shader_program);
        
        let loc = gl.GetAttribLocation(
            shader_program,
            CString::new("position").unwrap().as_ptr() as *const u8,
        );
        gl.EnableVertexAttribArray(loc.try_into().unwrap());
        gl.VertexAttribPointer(
            loc.try_into().unwrap(),
            3,
            GL_FLOAT,
            0,
            size_of::<Vertex>().try_into().unwrap(),
            0 as *const _,
        );

        // Limpeza dos shaders (não são mais necessários após a linkagem)
        gl.DeleteShader(vertex_shader);
        gl.DeleteShader(fragment_shader);

        // Configurações finais
        glfw.set_swap_interval(glfw::SwapInterval::Sync(1_u32));
        gl.ClearColor(0.7, 0.5, 0.3, 1.0);

        // Loop principal
        let mut radians = 0.0;
        while !window.should_close() {
            radians += 0.01;
            let mut rng = rand::rng();
            gl.Clear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

            // Configuração das transformações
            let loc_color = gl.GetUniformLocation(
                shader_program,
                CString::new("color").unwrap().as_ptr() as *const u8,
            );
            let loc = gl.GetUniformLocation(
                shader_program,
                CString::new("mat_transformation").unwrap().as_ptr() as *const u8,
            );
            
            let matriz_transformacao = IDENTITY_MATRIX
                .multiplication(&matriz_rotacao_x(45.0))
                .multiplication(&matriz_rotacao_y(45.0));
            
            gl.UniformMatrix4fv(
                loc.try_into().unwrap(),
                1,
                1,
                matriz_transformacao.to_matrix4fv().try_into().unwrap(),
            );

            // Renderização dos triângulos
            for triangle in (0..vertices_cilindro.len()).step_by(3) {
                if triangle <= (vertices_cilindro.len()/5) + 310 || 
                   triangle >= (vertices_cilindro.len()*4/5) - 100 {
                    gl.Uniform4f(loc_color.try_into().unwrap(), 0.2, 0.2, 0.2, 1.0);
                } else {
                    gl.Uniform4f(loc_color.try_into().unwrap(), 0.4, 0.4, 0.4, 1.0);
                }
                gl.DrawArrays(GL_TRIANGLES, triangle.try_into().unwrap(), 3);
            }

            // Tratamento de eventos
            for (_, event) in glfw::flush_messages(&events) {
                if let glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
                    window.set_should_close(true)
                }
            }

            window.swap_buffers();
            glfw.poll_events();
        }
    }
}

unsafe fn compile_shader(gl: &GlFns, shader_type: GLenum, source: &str) -> u32 {
    unsafe{
    let shader = gl.CreateShader(shader_type);
    assert_ne!(shader, 0);
    
    gl.ShaderSource(
        shader,
        1, 
        &(source.as_bytes().as_ptr().cast()),
        &(source.len().try_into().unwrap()),
    );
    gl.CompileShader(shader);

    let mut success = 0;
    gl.GetShaderiv(shader, GL_COMPILE_STATUS, &mut success);

    if success == 0 {
        let mut v: Vec<u8> = Vec::with_capacity(1024);
        let mut log_len = 0_i32;
        gl.GetShaderInfoLog(shader, 1024, &mut log_len, v.as_mut_ptr().cast());
        v.set_len(log_len.try_into().unwrap());
        panic!(
            "Erro na compilação do shader: {:?}",
            String::from_utf8_lossy(&v)
        );
    }
    
    shader
    }
}

unsafe fn create_shader_program(gl: &GlFns, vertex_shader: u32, fragment_shader: u32) -> u32 {
    unsafe{let program = gl.CreateProgram();
    gl.AttachShader(program, vertex_shader);
    gl.AttachShader(program, fragment_shader);
    gl.LinkProgram(program);

    let mut success = 0;
    gl.GetProgramiv(program, GL_LINK_STATUS, &mut success);
    if success == 0 {
        let mut v: Vec<u8> = Vec::with_capacity(1024);
        let mut log_len = 0_i32;
        gl.GetProgramInfoLog(program, 1024, &mut log_len, v.as_mut_ptr().cast());
        v.set_len(log_len.try_into().unwrap());
        panic!(
            "Erro da linkagem dos shaders no programa: {}",
            String::from_utf8_lossy(&v)
        );
    }
    
    program
    }
}
