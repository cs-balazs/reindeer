use crate::{backend::Backend, SHADERS};
use gl::types::{GLchar, GLint};
use std::{collections::HashSet, ffi::CString, ptr, str};

#[derive(Debug, Clone)]
pub struct Shader {
    pub id: <super::Context as Backend>::Program,
    pub uniforms: HashSet<String>,
}

impl Shader {
    pub fn new(name: &str) -> Shader {
        let id = compile_program(name);

        // TODO: Parse uniform names in Shader.uniforms

        Shader {
            id,
            uniforms: HashSet::new(),
        }
    }
}

impl super::super::shader::Shader<[f32; 3]> for Shader {
    fn set_uniform(&self, name: &str, input: [f32; 3]) {
        let loc: i32;

        unsafe {
            let c_str = CString::new(name.as_bytes()).unwrap();
            loc = gl::GetUniformLocation(self.id, c_str.as_ptr());
            gl::UseProgram(self.id);
            gl::Uniform3f(loc, input[0], input[1], input[2]);
        };
    }
}

impl super::super::shader::Shader<[[f32; 3]; 3]> for Shader {
    fn set_uniform(&self, name: &str, input: [[f32; 3]; 3]) {
        let loc: i32;

        unsafe {
            let c_str = CString::new(name.as_bytes()).unwrap();
            loc = gl::GetUniformLocation(self.id, c_str.as_ptr());
            gl::UseProgram(self.id);
            gl::UniformMatrix3fv(loc, 1, 0, &input[0][0]);
        };
    }
}

impl super::super::shader::Shader<[[f32; 4]; 4]> for Shader {
    fn set_uniform(&self, name: &str, input: [[f32; 4]; 4]) {
        let loc: i32;

        unsafe {
            let c_str = CString::new(name.as_bytes()).unwrap();
            loc = gl::GetUniformLocation(self.id, c_str.as_ptr());
            gl::UseProgram(self.id);
            gl::UniformMatrix4fv(loc, 1, 0, &input[0][0]);
        };
    }
}

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
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null::<i32>());
        gl::CompileShader(shader);

        // Get the compile status
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.fill(0);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character

            gl::GetShaderInfoLog(
                shader,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
            panic!(
                "{}",
                str::from_utf8(&buf).expect("ShaderInfoLog not valid utf8")
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
            buf.fill(0);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetProgramInfoLog(
                program,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
            panic!(
                "{}",
                str::from_utf8(&buf).expect("ProgramInfoLog not valid utf8")
            );
        }
    }
    program
}
