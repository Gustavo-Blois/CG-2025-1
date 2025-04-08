extern crate glfw;
use gl33::*;
use glfw::{Action, Context, Key};
use rand::prelude::*;
use std::ffi::CString;

mod alglin;
use alglin::*;
mod objetos;
use objetos::*;

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
        .create_window(1000, 1000, "Uma janela", glfw::WindowMode::Windowed)
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

        gl.BindBuffer(GL_ARRAY_BUFFER, vbo);

        //Obtendo os vertices e informacoes relevantes de cada objeto
        let (n_vertices_peso1, offset_peso_2, mut vertices_halter) = cria_halter(0.1, 0.2, 0.3);

        let mut vertices_pulldown = cria_pulldown(1.0, 0.05);
        let n_vertices_pulldown = vertices_pulldown.len();

        let (
            n_vertices_tronco_e_cabeca,
            n_vertices_antebraco,
            n_vertices_braco,
            mut vertices_pessoa,
        ) = cria_pessoa(0.5, 0.5);

        let mut vertices_pessoa2 = cria_pessoa2(0.5, 1.0);
        let n_vertices_pessoa2 = vertices_pessoa2.len();

        let mut vertices_banco = cria_banco(1.0, 0.2);
        let n_vertices_banco = vertices_banco.len();

        let (n_vertices_de_cima, mut vertices_pesos_pulldown) = cria_pesos_do_pulldown(0.5, 0.5);
        let n_vertices_peso_pulldown = vertices_pesos_pulldown.len();
        let mut lista_de_vertices: Vertices = Vec::new();
        let inicio_halter = lista_de_vertices.len();
        lista_de_vertices.append(&mut vertices_halter);

        let inicio_pulldown = lista_de_vertices.len();
        lista_de_vertices.append(&mut vertices_pulldown);

        let inicio_pessoa1 = lista_de_vertices.len();
        lista_de_vertices.append(&mut vertices_pessoa);

        let inicio_pessoa2 = lista_de_vertices.len();
        lista_de_vertices.append(&mut vertices_pessoa2);

        let inicio_banco = lista_de_vertices.len();
        lista_de_vertices.append(&mut vertices_banco);

        let inicio_pesos_pulldown = lista_de_vertices.len();
        lista_de_vertices.append(&mut vertices_pesos_pulldown);

        //Enviando os vertices obtidos para o buffer;
        gl.BufferData(
            GL_ARRAY_BUFFER,
            (lista_de_vertices.len() * std::mem::size_of::<[f32; 3]>()) as isize,
            lista_de_vertices.as_ptr().cast(),
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
        gl.Enable(GL_DEPTH_TEST);
        gl.ClearColor(0.7, 0.9, 0.7, 1.0);

        // Loop principal
        let mut poligon_mode = 0;
        let mut comando_pessoa1: i8 = 0;
        let mut comando_pessoa2: i8 = 8;
        while !window.should_close() {
            if poligon_mode == 0 {
                gl.PolygonMode(GL_FRONT_AND_BACK, GL_FILL);
            } else {
                gl.PolygonMode(GL_FRONT_AND_BACK, GL_LINE);
            }

            gl.Clear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
            desenha_pessoa1(
                &gl,
                inicio_pessoa1,
                n_vertices_tronco_e_cabeca,
                n_vertices_antebraco,
                n_vertices_braco,
                shader_program,
                comando_pessoa1,
            );

            desenha_pulldown(
                &gl,
                inicio_pulldown,
                n_vertices_pulldown,
                shader_program,
                comando_pessoa1,
            );
            desenha_pesos_pulldown(
                &gl,
                inicio_pesos_pulldown,
                n_vertices_de_cima,
                n_vertices_peso_pulldown,
                shader_program,
                comando_pessoa1,
            );

            desenha_banco(&gl, inicio_banco, n_vertices_banco, shader_program);

            desenha_pessoa2(
                &gl,
                inicio_pessoa2,
                n_vertices_pessoa2,
                shader_program,
                comando_pessoa2,
            );

            // eventos
            for (_, event) in glfw::flush_messages(&events) {
                if let glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
                    window.set_should_close(true)
                }
                if let glfw::WindowEvent::Key(Key::P, _, Action::Press, _) = event {
                    poligon_mode = poligon_mode ^ 1;
                }
                if let glfw::WindowEvent::Key(Key::H, _, Action::Press, _) = event {
                    if comando_pessoa1 < 0 {
                        comando_pessoa1 += 1;
                    }
                }
                if let glfw::WindowEvent::Key(Key::L, _, Action::Press, _) = event {
                    if comando_pessoa1 > -3 {
                        comando_pessoa1 -= 1;
                    }
                }
            }

            window.swap_buffers();
            glfw.poll_events();
        }
    }
}
unsafe fn desenha_pessoa2(gl: &GlFns, start: usize, size: usize, shader_program: u32, comando: i8) {
    unsafe {
        let loc = gl.GetUniformLocation(
            shader_program,
            CString::new("mat_transformation").unwrap().as_ptr() as *const u8,
        );

        let loc_color = gl.GetUniformLocation(
            shader_program,
            CString::new("color").unwrap().as_ptr() as *const u8,
        );
        let matriz_transformacao = IDENTITY_MATRIX;
        gl.UniformMatrix4fv(
            loc.try_into().unwrap(),
            1,
            1,
            matriz_transformacao.to_matrix4fv().try_into().unwrap(),
        );
        gl.Uniform4f(loc_color.try_into().unwrap(), 0.2, 0.2, 0.2, 1.0);

        gl.DrawArrays(
            GL_TRIANGLES,
            start.try_into().unwrap(),
            size.try_into().unwrap(),
        );
    }
}

unsafe fn desenha_pulldown(
    gl: &GlFns,
    start: usize,
    size: usize,
    shader_program: u32,
    comando: i8,
) {
    unsafe {
        let loc = gl.GetUniformLocation(
            shader_program,
            CString::new("mat_transformation").unwrap().as_ptr() as *const u8,
        );

        let loc_color = gl.GetUniformLocation(
            shader_program,
            CString::new("color").unwrap().as_ptr() as *const u8,
        );
        let matriz_transformacao = IDENTITY_MATRIX
            .multiplication(&matriz_escala(1.0, 0.650, 0.0))
            .multiplication(&matriz_translacao(
                0.25,
                0.7 + ((comando as f32) / 8.0),
                0.0,
            ));
        gl.UniformMatrix4fv(
            loc.try_into().unwrap(),
            1,
            1,
            matriz_transformacao.to_matrix4fv().try_into().unwrap(),
        );
        gl.Uniform4f(loc_color.try_into().unwrap(), 0.2, 0.2, 0.2, 1.0);

        gl.DrawArrays(
            GL_TRIANGLES,
            start.try_into().unwrap(),
            size.try_into().unwrap(),
        );
    }
}

unsafe fn desenha_pesos_pulldown(
    gl: &GlFns,
    start: usize,
    size_pesos_de_cima: usize,
    size: usize,
    shader_program: u32,
    comando: i8,
) {
    unsafe {
        desenha_pesos_de_cima(&gl, start, size_pesos_de_cima, shader_program, comando);
        desenha_pesos_de_baixo(&gl, start + size_pesos_de_cima, size, shader_program);
    }
}

unsafe fn desenha_pesos_de_baixo(gl: &GlFns, start: usize, size: usize, shader_program: u32) {
    unsafe {
        let loc = gl.GetUniformLocation(
            shader_program,
            CString::new("mat_transformation").unwrap().as_ptr() as *const u8,
        );

        let loc_color = gl.GetUniformLocation(
            shader_program,
            CString::new("color").unwrap().as_ptr() as *const u8,
        );
        let matriz_transformacao = IDENTITY_MATRIX
            .multiplication(&matriz_translacao(0.50, 0.0, 0.0))
            .multiplication(&matriz_escala(0.5, 0.50, 0.0));
        gl.UniformMatrix4fv(
            loc.try_into().unwrap(),
            1,
            1,
            matriz_transformacao.to_matrix4fv().try_into().unwrap(),
        );
        gl.Uniform4f(loc_color.try_into().unwrap(), 0.2, 0.2, 0.2, 1.0);

        gl.DrawArrays(
            GL_TRIANGLES,
            start.try_into().unwrap(),
            size.try_into().unwrap(),
        );
    }
}

unsafe fn desenha_pesos_de_cima(
    gl: &GlFns,
    start: usize,
    size: usize,
    shader_program: u32,
    comando: i8,
) {
    unsafe {
        let loc = gl.GetUniformLocation(
            shader_program,
            CString::new("mat_transformation").unwrap().as_ptr() as *const u8,
        );

        let loc_color = gl.GetUniformLocation(
            shader_program,
            CString::new("color").unwrap().as_ptr() as *const u8,
        );
        let matriz_transformacao = IDENTITY_MATRIX
            .multiplication(&matriz_translacao(0.5, -((comando as f32) / 16.0), 0.0))
            .multiplication(&matriz_escala(0.5, 0.50, 0.0));
        gl.UniformMatrix4fv(
            loc.try_into().unwrap(),
            1,
            1,
            matriz_transformacao.to_matrix4fv().try_into().unwrap(),
        );
        gl.Uniform4f(loc_color.try_into().unwrap(), 0.2, 0.2, 0.2, 1.0);

        gl.DrawArrays(
            GL_TRIANGLES,
            start.try_into().unwrap(),
            size.try_into().unwrap(),
        );
    }
}

unsafe fn desenha_banco(gl: &GlFns, start: usize, size: usize, shader_program: u32) {
    unsafe {
        let loc = gl.GetUniformLocation(
            shader_program,
            CString::new("mat_transformation").unwrap().as_ptr() as *const u8,
        );

        let loc_color = gl.GetUniformLocation(
            shader_program,
            CString::new("color").unwrap().as_ptr() as *const u8,
        );
        let matriz_transformacao = IDENTITY_MATRIX
            .multiplication(&matriz_translacao(0.25, -0.5, 0.0))
            .multiplication(&matriz_rotacao_y(90.0));
        gl.UniformMatrix4fv(
            loc.try_into().unwrap(),
            1,
            1,
            matriz_transformacao.to_matrix4fv().try_into().unwrap(),
        );
        gl.Uniform4f(loc_color.try_into().unwrap(), 0.2, 0.2, 0.2, 1.0);

        gl.DrawArrays(
            GL_TRIANGLES,
            start.try_into().unwrap(),
            size.try_into().unwrap(),
        );
    }
}

unsafe fn desenha_pessoa1(
    gl: &GlFns,
    start: usize,
    size_tronco_e_cabeca: usize,
    size_antebraco: usize,
    size_braco: usize,
    shader_program: u32,
    comando: i8,
) {
    unsafe {
        let matriz_base =
            matriz_escala(0.5, 0.5, 0.5).multiplication(&matriz_translacao(0.5, 0.5, 0.0));
        desenha_bracos(
            &gl,
            start,
            size_antebraco,
            size_braco,
            shader_program,
            comando,
            &matriz_base,
        );
        desenha_tronco_e_cabeca(
            &gl,
            start + size_antebraco + size_braco,
            size_tronco_e_cabeca,
            shader_program,
            &matriz_base,
        );
    }
}
unsafe fn desenha_tronco_e_cabeca(
    gl: &GlFns,
    start: usize,
    size: usize,
    shader_program: u32,
    matriz_base: &V4Matrix,
) {
    unsafe {
        let loc = gl.GetUniformLocation(
            shader_program,
            CString::new("mat_transformation").unwrap().as_ptr() as *const u8,
        );

        let loc_color = gl.GetUniformLocation(
            shader_program,
            CString::new("color").unwrap().as_ptr() as *const u8,
        );
        let matriz_transformacao = IDENTITY_MATRIX.multiplication(&matriz_base);
        gl.UniformMatrix4fv(
            loc.try_into().unwrap(),
            1,
            1,
            matriz_transformacao.to_matrix4fv().try_into().unwrap(),
        );
        gl.Uniform4f(loc_color.try_into().unwrap(), 0.35, 0.24, 0.20, 1.0);

        gl.DrawArrays(
            GL_TRIANGLES,
            start.try_into().unwrap(),
            size.try_into().unwrap(),
        );
    }
}
unsafe fn desenha_bracos(
    gl: &GlFns,
    start: usize,
    size_antebraco: usize,
    size_braco: usize,
    shader_program: u32,
    comando: i8,
    matriz_base: &V4Matrix,
) {
    unsafe {
        desenha_antebraco(
            &gl,
            start,
            size_antebraco,
            shader_program,
            comando,
            &matriz_base,
        );
        desenha_braco(
            &gl,
            start + size_antebraco,
            size_braco,
            shader_program,
            comando,
            &matriz_base,
        );
    }
}
unsafe fn desenha_braco(
    gl: &GlFns,
    start: usize,
    size: usize,
    shader_program: u32,
    comando: i8,
    matriz_base: &V4Matrix,
) {
    unsafe {
        let loc = gl.GetUniformLocation(
            shader_program,
            CString::new("mat_transformation").unwrap().as_ptr() as *const u8,
        );

        let loc_color = gl.GetUniformLocation(
            shader_program,
            CString::new("color").unwrap().as_ptr() as *const u8,
        );
        let matriz_transformacao = IDENTITY_MATRIX
            .multiplication(&matriz_base)
            .multiplication(&matriz_translacao(0.0, (comando as f32) / 8.0, 0.0));

        gl.UniformMatrix4fv(
            loc.try_into().unwrap(),
            1,
            1,
            matriz_transformacao.to_matrix4fv().try_into().unwrap(),
        );
        gl.Uniform4f(loc_color.try_into().unwrap(), 0.35, 0.24, 0.20, 1.0);

        gl.DrawArrays(
            GL_TRIANGLES,
            start.try_into().unwrap(),
            size.try_into().unwrap(),
        );
    }
}
unsafe fn desenha_antebraco(
    gl: &GlFns,
    start: usize,
    size: usize,
    shader_program: u32,
    comando: i8,
    matriz_base: &V4Matrix,
) {
    unsafe {
        let loc = gl.GetUniformLocation(
            shader_program,
            CString::new("mat_transformation").unwrap().as_ptr() as *const u8,
        );

        let loc_color = gl.GetUniformLocation(
            shader_program,
            CString::new("color").unwrap().as_ptr() as *const u8,
        );
        let matriz_transformacao = IDENTITY_MATRIX
            .multiplication(&matriz_base)
            .multiplication(&matriz_translacao(0.0, (comando as f32) / 8.0, 0.0));

        gl.UniformMatrix4fv(
            loc.try_into().unwrap(),
            1,
            1,
            matriz_transformacao.to_matrix4fv().try_into().unwrap(),
        );
        gl.Uniform4f(loc_color.try_into().unwrap(), 0.35, 0.24, 0.20, 1.0);

        gl.DrawArrays(
            GL_TRIANGLES,
            start.try_into().unwrap(),
            size.try_into().unwrap(),
        );
    }
}

unsafe fn compile_shader(gl: &GlFns, shader_type: GLenum, source: &str) -> u32 {
    unsafe {
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
    unsafe {
        let program = gl.CreateProgram();
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
