use gl33::*;
use std::{env, fs, path::PathBuf};

pub fn get_vertex_shader() -> String {
    fs::read_to_string(
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("shaders/vertex_shader.vs"),
    )
    .expect("arquivo não encontrado")
}

pub fn get_fragment_shader() -> String {
    fs::read_to_string(
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("shaders/fragment_shader.fs"),
    )
    .expect("arquivo não encontrado")
}

pub unsafe fn compile_shader(gl: &GlFns, shader_type: GLenum) -> u32 {
    unsafe {
        let source: String;
        if shader_type == GL_VERTEX_SHADER {
            source = get_vertex_shader();
        } else if shader_type == GL_FRAGMENT_SHADER {
            source = get_fragment_shader();
        } else {
            panic!("source está vazia")
        }
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

pub unsafe fn create_shader_program(gl: &GlFns, vertex_shader: u32, fragment_shader: u32) -> u32 {
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
