extern crate glfw;
use glfw::{Action, Context, Key};
use gl33::*;
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
    // Muda a cor do fundo
    unsafe {gl.ClearColor(0.3,0.3,0.3,1.0);};
    
    // define o vertex array object
    unsafe {
        let mut vao = 0;
        gl.GenVertexArrays(1, &mut vao);
        //o vertex array object não pode ser zero depois dessa operação
        assert_ne!(vao,0);
    }

    //define o vertex buffer object

    unsafe {
        let mut vbo = 0;
        gl.GenBuffers(1, &mut vbo);
        assert_ne!(vbo,0);
    // Vincula o vbo
        gl.BindBuffer(GL_ARRAY_BUFFER,vbo);
    }

    // Define os vértices
    type Vertex = [f32;3];
    const VERTICES: [Vertex; 3]=
        [[-0.5,-0.5,0.0],[0.5,-0.5,0.0],[0.0,0.5,0.0]];
    
    // Envia nossos vértices pro array buffer
    unsafe {
        gl.BufferData(GL_ARRAY_BUFFER,
            size_of_val(&VERTICES) as isize,
            VERTICES.as_ptr().cast(),
            GL_STATIC_DRAW,
        );
    // Dados pro vertex attribute pointer
        gl.VertexAttribPointer(
            0,
            3,
            GL_FLOAT,
            0, //GL_FALSE
            size_of::<Vertex>().try_into().unwrap(), //Isso passa a informação do tamanho do Vertex
                                                   //como um isize ao invés de um usize
            std::ptr::null(),
        );
        gl.EnableVertexAttribArray(0);

    //Criação do vertex shader program object
        let vertex_shader = gl.CreateShader(GL_VERTEX_SHADER);
        assert_ne!(vertex_shader,0);
        const VERT_SHADER: &str = r#"#version 330 core
        layout (location = 0) in vec3 pos;
        void main() {
        gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
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

    void main(){
        final_color = vec4(1.0,0.5,0.2,1.0);
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

    // Agora que os shaders foram linkados, podemos deletar eles
        gl.DeleteShader(vertex_shader);
        gl.DeleteShader(fragment_shader);
        }
    // Vsync
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1_u32));
    //loop até usuário fechar a janela
    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}",event);
            if let glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
                window.set_should_close(true)
            }
        }
        unsafe{
        gl.Clear(GL_COLOR_BUFFER_BIT);
        gl.DrawArrays(GL_TRIANGLES, 0, 3);
        }
        window.swap_buffers();
    }

}
