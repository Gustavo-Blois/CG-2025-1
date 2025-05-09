extern crate glfw;
use gl33::global_loader::*;
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
        .create_window(800, 600, "Uma janela", glfw::WindowMode::Windowed)
        .expect("Falha em criar uma janela glfw");

    window.make_current();
    window.set_key_polling(true);

    unsafe {
        //carrega o opengl globalmente
        load_global_gl(&|p| {
            glfw.get_proc_address_raw(&glfw::string_from_c_str(p as *const _)) as *const _
        });
        let mut lista_de_vertices: Vec<[f32; 3]> = Vec::new();
        let mut lista_de_texturas: Vec<[f32; 2]> = Vec::new();

        let (vertice_inicial_spiderman, n_vertices_spiderman) = load_obj_and_texture(
            "spiderman",
            ["spiderman/spiderman.png".to_string()].to_vec(),
            &mut lista_de_vertices,
            &mut lista_de_texturas,
        );
        println!("vertice_inicial_spiderman = {}\nvertice_final_spiderman = {}",vertice_inicial_spiderman,n_vertices_spiderman);
        // Configuração de VAO e VBO
        let mut vao = 0;
        glGenVertexArrays(1, &mut vao);
        assert_ne!(vao, 0);

        let mut vbo = [0; 2];
        // requisitamos dois slots
        glGenBuffers(2, vbo.as_mut_ptr());

        //interrompemos se vbo continua igual a zero
        assert_ne!(vbo[0], 0);

        glBindBuffer(GL_ARRAY_BUFFER, vbo[0]);
        //Enviando os vertices obtidos para o buffer;
        glBufferData(
            GL_ARRAY_BUFFER,
            (lista_de_vertices.len() * std::mem::size_of::<[f32; 3]>()) as isize,
            lista_de_vertices.as_ptr().cast(),
            GL_STATIC_DRAW,
        );

        // Criação e compilação dos shaders
        let vertex_shader = compile_shader(GL_VERTEX_SHADER);
        let fragment_shader = compile_shader(GL_FRAGMENT_SHADER);

        // Criação do programa de shader
        let shader_program = create_shader_program(vertex_shader, fragment_shader);

        // Configuração dos atributos de vértice
        glBindVertexArray(vao);
        glUseProgram(shader_program);

        //Enviando coordenadas de vertice pra gpu
        let loc_vertices = glGetAttribLocation(
            shader_program,
            CString::new("position").unwrap().as_ptr() as *const u8,
        );
        glEnableVertexAttribArray(loc_vertices.try_into().unwrap());
        glVertexAttribPointer(
            loc_vertices.try_into().unwrap(),
            3,
            GL_FLOAT,
            0,
            size_of::<Vertex>().try_into().unwrap(),
            0 as *const _,
        );

        //enviando coordenadas de textura para a gpu
        glBindBuffer(GL_ARRAY_BUFFER, vbo[1]);
        glBufferData(
            GL_ARRAY_BUFFER,
            (lista_de_texturas.len() * std::mem::size_of::<[f32; 2]>()) as isize,
            lista_de_texturas.as_ptr().cast(),
            GL_STATIC_DRAW,
        );
        let loc_texture_coord = glGetAttribLocation(
            shader_program,
            CString::new("texture_coord").unwrap().as_ptr() as *const u8,
        );
        glEnableVertexAttribArray(loc_texture_coord.try_into().unwrap());
        glVertexAttribPointer(
            loc_texture_coord.try_into().unwrap(),
            2,
            GL_FLOAT,
            0,
            size_of::<[f32; 2]>().try_into().unwrap(),
            0 as *const _,
        );
        // Limpeza dos shaders (não são mais necessários após a linkagem)
        glDeleteShader(vertex_shader);
        glDeleteShader(fragment_shader);

        // Configurações finais
        glfw.set_swap_interval(glfw::SwapInterval::Sync(1_u32));
        glEnable(GL_DEPTH_TEST);
        glClearColor(1.0, 1.0, 1.0, 1.0);
        glViewport(0,0,800,600);
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

            //model
            desenha_objeto(
                vertice_inicial_spiderman,
                n_vertices_spiderman,
                shader_program,
                0,
                0.0,
                0.0,
                0.0,
                1.0,
                0.0,
                0.0,
                -20.0,
                1.5,
                1.5,
                1.5,
            );

            //matriz view
            let mat_view = view(Camera::new());
            let loc_view = glGetUniformLocation(
                shader_program,
                CString::new("view").unwrap().as_ptr() as *const u8,
            );
            glUniformMatrix4fv(loc_view, 1, 1, mat_view.to_matrix4fv());

            //matriz projection
            let largura = 800.0;
            let altura = 600.0;
            let aspect_ratio = largura / altura;
            let fov_y = 45.0_f32.to_radians();
            let near = 0.1;
            let far = 100.0;

            let top = (fov_y / 2.0).tan() * near;
            let bottom = -top;
            let right = top * aspect_ratio;
            let left = -right;

            let mat_projection = projection(near, far, top, bottom, right, left);
            let loc_projection = glGetUniformLocation(
                shader_program,
                CString::new("projection").unwrap().as_ptr() as *const u8,
            );
            glUniformMatrix4fv(loc_projection, 1, 1, mat_projection.to_matrix4fv());

            window.swap_buffers();
            glfw.poll_events();
        }
    }
}
