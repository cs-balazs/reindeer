use gl::types::{GLchar, GLint};
use std::{ffi::CString, ptr, str};

use crate::renderer::SHADERS;

pub fn compile_program(name: &str) -> u32 {
    let fragment_shader = compile_shader(gl::FRAGMENT_SHADER, name).unwrap();
    let vertex_shader = compile_shader(gl::VERTEX_SHADER, name).unwrap();

    link_program(vertex_shader, fragment_shader)
}

fn compile_shader(shader_type: u32, name: &str) -> Result<u32, String> {
    let shader: u32;

    let shader_source = SHADERS[format!(
        "{}.{}.glsl",
        name,
        if shader_type == gl::FRAGMENT_SHADER {
            "frag"
        } else {
            "vert"
        }
    )
    .as_str()];

    unsafe {
        let c_str = CString::new(shader_source.as_bytes()).unwrap();
        shader = gl::CreateShader(shader_type);
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), 0 as *const i32);
        gl::CompileShader(shader);

        // Get the compile status
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderInfoLog(
                shader,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
            panic!(
                "{}",
                str::from_utf8(&buf)
                    .ok()
                    .expect("ShaderInfoLog not valid utf8")
            );
        }
    }

    Ok(shader)
}

fn link_program(vertex_shader: u32, fragment_shader: u32) -> u32 {
    let program: u32;
    unsafe {
        program = gl::CreateProgram();
        gl::AttachShader(program, fragment_shader);
        gl::AttachShader(program, vertex_shader);
        gl::LinkProgram(program);
        gl::ValidateProgram(program);
        // gl::DeleteShader(vertex_shader);
        // gl::DeleteShader(fragment_shader);

        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetProgramInfoLog(
                program,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
            panic!(
                "{}",
                str::from_utf8(&buf)
                    .ok()
                    .expect("ProgramInfoLog not valid utf8")
            );
        }
    }
    program
}
