extern crate glfw;
use gl33::*;
use glfw::{Action, Context, Key};
use std::ffi::CString;

use projeto_cg::alglin::*;
use projeto_cg::mvp::*;
use projeto_cg::objetos::*;
use projeto_cg::shaders::*;

fn main() {
    use glfw::fail_on_errors;

    // Inicialização GLFW e janela
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();
    let (mut window, events) = glfw
        .create_window(1000, 1000, "Uma janela", glfw::WindowMode::Windowed)
        .expect("Falha em criar uma janela glfw");

    window.make_current();
    window.set_key_polling(true);



    unsafe {
        //carrega o opengl globalmente
        load_global_gl(&|p| {
            glfw.get_proc_address_raw(&glfw::string_from_c_str(p as *const _)) as *const _
        });
        // Configuração de VAO e VBO
        let mut vao = 0;
        glGenVertexArrays(1, &mut vao);
        assert_ne!(vao, 0);

        let mut vbo = 0;
        glGenBuffers(1, &mut vbo);
        assert_ne!(vbo, 0);

        glBindBuffer(GL_ARRAY_BUFFER, vbo);

        let miranha = cria_objeto(
            "/home/gundrjsse/projetos/rust/012025/CG-2025-1/objetos/spiderman/spiderman.obj"
                .to_string(),
        );

        //Enviando os vertices obtidos para o buffer;
        glBufferData(
            GL_ARRAY_BUFFER,
            (miranha.vertices.len() * std::mem::size_of::<[f32; 3]>()) as isize,
            miranha.vertices.as_ptr().cast(),
            GL_DYNAMIC_DRAW,
        );

        // Criação e compilação dos shaders
        let vertex_shader = compile_shader(&gl, GL_VERTEX_SHADER);
        let fragment_shader = compile_shader(&gl, GL_FRAGMENT_SHADER);

        // Criação do programa de shader
        let shader_program = create_shader_program(&gl, vertex_shader, fragment_shader);

        // Configuração dos atributos de vértice
        glBindVertexArray(vao);
        glUseProgram(shader_program);

        let loc = glGetAttribLocation(
            shader_program,
            CString::new("position").unwrap().as_ptr() as *const u8,
        );
        glEnableVertexAttribArray(loc.try_into().unwrap());
        glVertexAttribPointer(
            loc.try_into().unwrap(),
            3,
            GL_FLOAT,
            0,
            size_of::<Vertex>().try_into().unwrap(),
            0 as *const _,
        );

        // Limpeza dos shaders (não são mais necessários após a linkagem)
        glDeleteShader(vertex_shader);
        glDeleteShader(fragment_shader);

        // Configurações finais
        glfw.set_swap_interval(glfw::SwapInterval::Sync(1_u32));
        glEnable(GL_DEPTH_TEST);
        glClearColor(0.749, 0.845, 1.0, 1.0);

        // Loop principal
        let mut poligon_mode = 0;
        while !window.should_close() {
            if poligon_mode == 0 {
                glPolygonMode(GL_FRONT_AND_BACK, GL_FILL);
            } else {
                glPolygonMode(GL_FRONT_AND_BACK, GL_LINE);
            }

            glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
            // eventos
            for (_, event) in glfw::flush_messages(&events) {
                if let glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
                    window.set_should_close(true)
                }
                if let glfw::WindowEvent::Key(Key::P, _, Action::Press, _) = event {
                    poligon_mode = poligon_mode ^ 1;
                }
            }

            window.swap_buffers();
            glfw.poll_events();
        }
    }
}
