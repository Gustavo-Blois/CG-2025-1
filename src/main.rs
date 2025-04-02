extern crate glfw;
use gl33::*;
use glfw::{Action, Context, Key};
use rand::prelude::*;
use std::ffi::{CString, c_float};
mod cilindro;
use cilindro::cria_cilindro;
fn main() {
    use glfw::fail_on_errors;
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();

    // Cria uma janela glfw
    let (mut window, events) = glfw
        .create_window(800, 600, "Uma janela", glfw::WindowMode::Windowed)
        .expect("Falha em criar uma janela glfw");
    window.make_current();
    window.set_key_polling(true);
    // Dado um contexto (vindo do glfw) e um processo, do opengl, obtém o endereço do processo e
    // carrega o opengl com esse endereço
    let gl = unsafe {
        GlFns::load_from(&|p| {
            glfw.get_proc_address_raw(&glfw::string_from_c_str(p as *const _)) as *const _
        })
        .unwrap()
    };

    // define o vertex array object
    unsafe {
        let mut vao = 0;
        gl.GenVertexArrays(1, &mut vao);
        //o vertex array object não pode ser zero depois dessa operação
        assert_ne!(vao, 0);

        //define o vertex buffer object

        let mut vbo = 0;
        gl.GenBuffers(1, &mut vbo);
        assert_ne!(vbo, 0);
        // Vincula o vbo
        gl.BindBuffer(GL_ARRAY_BUFFER, vbo);

        // Vertices:
        let vertices_cilindro = cria_cilindro(0.1, 0.9);
        let n_vertices_cilindro = vertices_cilindro.len();
        // Envia nossos vértices pro array buffer
        gl.BufferData(
            GL_ARRAY_BUFFER,
            (vertices_cilindro.len() * std::mem::size_of::<[f32; 3]>()) as isize,
            vertices_cilindro.as_ptr().cast(),
            GL_DYNAMIC_DRAW,
        );

        //Criação do vertex shader program object
        let vertex_shader = gl.CreateShader(GL_VERTEX_SHADER);
        assert_ne!(vertex_shader, 0);
        const VERT_SHADER: &str = r#"#version 330 core
        attribute vec3 position;
        uniform mat4 mat_transformation;
        void main() {
        gl_Position = mat_transformation*vec4(position, 1.0);
        }"#;
        // envia o código fonte do shader para o nosso vspo

        gl.ShaderSource(
            vertex_shader,
            1, // numero de elementos dos vetores string e tamanho de string. Eu não sei o porquê
            // disso, mas está na documentação https://registry.khronos.org/OpenGL-Refpages/gl4/html/glShaderSource.xhtml
            &(VERT_SHADER.as_bytes().as_ptr().cast()), // a string do código fonte
            &(VERT_SHADER.len().try_into().unwrap()),  // o tamanho da string
        );
        // Compila o vertex shader
        gl.CompileShader(vertex_shader);

        // Verifica se a compilação deu certo

        let mut success = 0;
        gl.GetShaderiv(vertex_shader, GL_COMPILE_STATUS, &mut success);

        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024); //Vetor que armazena o log de erro
            let mut log_len = 0_i32;
            gl.GetShaderInfoLog(vertex_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());
            panic!(
                "Erro na compilação do vertex shader: {:?}",
                String::from_utf8_lossy(&v)
            );
        }

        // Fragment Shader

        let fragment_shader = gl.CreateShader(GL_FRAGMENT_SHADER);
        assert_ne!(fragment_shader, 0);

        const FRAG_SHADER: &str = r#"
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
            gl.GetShaderInfoLog(fragment_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());
            panic!(
                "Erro na compilação do fragment shader: {:?}",
                String::from_utf8_lossy(&v)
            );
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
            gl.GetProgramInfoLog(shader_program, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());
            panic!(
                "Erro da linkagem dos shaders no programa: {}",
                String::from_utf8_lossy(&v)
            );
        }
        gl.BindVertexArray(vao);
        gl.UseProgram(shader_program);
        // Dados pro vertex attribute pointer
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

        // Agora que os shaders foram linkados, podemos deletar eles
        gl.DeleteShader(vertex_shader);
        gl.DeleteShader(fragment_shader);
        // Vsync
        glfw.set_swap_interval(glfw::SwapInterval::Sync(1_u32));

        // position

        // Muda a cor do fundo
        gl.ClearColor(0.3, 0.3, 0.3, 1.0);

        //Matriz de rotação
        //loop até usuário fechar a janela
        let mut radians = 0.0;
        while !window.should_close() {
            radians += 10.0;
            let mut rng = rand::rng();
            gl.Clear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
            let loc_color = gl.GetUniformLocation(
                shader_program,
                CString::new("color").unwrap().as_ptr() as *const u8,
            );
            let loc = gl.GetUniformLocation(
                shader_program,
                CString::new("mat_transformation").unwrap().as_ptr() as *const u8,
            );
            let uma_matriz_rotacao_y = matriz_rotacao_y(radians * rng.random::<f32>());
            let matriz_transformacao = IDENTITY_MATRIX
                .multiplication(&uma_matriz_rotacao_y)
                .multiplication(&matriz_rotacao_x(radians * rng.random::<f32>()));
            gl.UniformMatrix4fv(
                loc.try_into().unwrap(),
                1,
                1,
                matriz_transformacao.to_matrix4fv().try_into().unwrap(),
            );
            let r: f32 = rng.random();
            let g: f32 = rng.random();
            let b: f32 = rng.random();

            for triangle in (0..vertices_cilindro.len()).step_by(3) {
                // println!("{:?}",triangle);
                println!("{r},{g},{b}");
                gl.Uniform4f(loc_color.try_into().unwrap(), r, g, b, 1.0);
                gl.DrawArrays(GL_TRIANGLES, triangle.try_into().unwrap(), 3);
            }

            for (_, event) in glfw::flush_messages(&events) {
                println!("{:?}", event);
                if let glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
                    window.set_should_close(true)
                }
            }
            window.swap_buffers();
            glfw.poll_events();
        }
    }
}
type Vertex = [f32; 3];
type V4Matrix = [[f32; 4]; 4];
const IDENTITY_MATRIX: V4Matrix = [
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
];
pub trait Matrix4fvTransformable {
    fn to_matrix4fv(&self) -> *const c_float;
}

pub trait V4MatrixUtils {
    fn multiplication(&self, another_matrix: &V4Matrix) -> V4Matrix;
}

impl V4MatrixUtils for V4Matrix {
    fn multiplication(&self, another_matrix: &V4Matrix) -> V4Matrix {
        let mut result: V4Matrix = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                let mut soma: f32 = 0.0;
                for k in 0..4 {
                    soma += self[i][k] * another_matrix[k][j];
                }
                result[i][j] = soma;
            }
        }
        return result;
    }
}

fn matriz_rotacao_x(angulo: f32) -> V4Matrix {
    [
        [1.0, 0.0, 0.0, 0.0],
        [
            0.0,
            angulo.to_radians().cos(),
            -angulo.to_radians().sin(),
            0.0,
        ],
        [
            0.0,
            angulo.to_radians().sin(),
            angulo.to_radians().cos(),
            0.0,
        ],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

fn matriz_rotacao_y(angulo: f32) -> V4Matrix {
    [
        [
            angulo.to_radians().cos(),
            0.0,
            -angulo.to_radians().sin(),
            0.0,
        ],
        [0.0, 1.0, 0.0, 0.0],
        [
            angulo.to_radians().sin(),
            0.0,
            angulo.to_radians().cos(),
            0.0,
        ],
        [0.0, 0.0, 0.0, 1.0],
    ]
}
fn matriz_rotacao_z(angulo: f32) -> V4Matrix {
    [
        [
            angulo.to_radians().cos(),
            -angulo.to_radians().sin(),
            0.0,
            0.0,
        ],
        [
            angulo.to_radians().sin(),
            angulo.to_radians().cos(),
            0.0,
            0.0,
        ],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

impl Matrix4fvTransformable for V4Matrix {
    fn to_matrix4fv(&self) -> *const c_float {
        self.as_ptr() as *const c_float
    }
}
