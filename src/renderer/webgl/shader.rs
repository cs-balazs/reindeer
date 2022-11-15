use std::{collections::HashSet, ffi::CString};

use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};

use crate::renderer::{CTX, SHADERS};

#[derive(Debug)]
pub struct Shader {
    pub id: super::Program,
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
        let loc = CTX
            .context
            .borrow()
            .context
            .get_uniform_location(&self.id, name)
            .expect(format!("Failed to find uniform location: {}", name).as_str());
        CTX.context.borrow().context.use_program(Some(&self.id));
        CTX.context
            .borrow()
            .context
            .uniform3f(Some(&loc), input[0], input[1], input[2]);
    }
}

impl super::super::shader::Shader<[[f32; 3]; 3]> for Shader {
    fn set_uniform(&self, name: &str, input: [[f32; 3]; 3]) {
        let loc = CTX
            .context
            .borrow()
            .context
            .get_uniform_location(&self.id, name)
            .expect(format!("Failed to find uniform location: {}", name).as_str());
        CTX.context.borrow().context.use_program(Some(&self.id));

        let input: [f32; 9] = [
            input[0][0],
            input[0][1],
            input[0][2],
            input[1][0],
            input[1][1],
            input[1][2],
            input[2][0],
            input[2][1],
            input[2][2],
        ];
        CTX.context
            .borrow()
            .context
            .uniform_matrix3fv_with_f32_array(Some(&loc), false, &input);
    }
}

pub fn compile_program(name: &str) -> WebGlProgram {
    let fragment_shader = compile_shader(WebGl2RenderingContext::FRAGMENT_SHADER, name)
        .expect("Compiling fragmet shader failed");

    let vertex_shader = compile_shader(WebGl2RenderingContext::VERTEX_SHADER, name)
        .expect("Compiling vertex shader failed");

    link_program(&vertex_shader, &fragment_shader)
}

fn compile_shader(shader_type: u32, name: &str) -> Result<WebGlShader, String> {
    let shader_source = SHADERS[format!(
        "{}.{}.glsl",
        name,
        if shader_type == WebGl2RenderingContext::FRAGMENT_SHADER {
            "frag"
        } else {
            "vert"
        }
    )
    .as_str()];

    let shader = CTX
        .context
        .borrow()
        .context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    CTX.context
        .borrow()
        .context
        .shader_source(&shader, shader_source);
    CTX.context.borrow().context.compile_shader(&shader);

    if CTX
        .context
        .borrow()
        .context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        if let Some(err) = CTX.context.borrow().context.get_shader_info_log(&shader) {
            let array = js_sys::Array::new();
            array.push(&err.into());
            web_sys::console::log(&array);
        }

        Err(CTX
            .context
            .borrow()
            .context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

fn link_program(vertex_shader: &WebGlShader, fragment_shader: &WebGlShader) -> WebGlProgram {
    let program = CTX.context.borrow().context.create_program().unwrap();

    CTX.context
        .borrow()
        .context
        .attach_shader(&program, vertex_shader);
    CTX.context
        .borrow()
        .context
        .attach_shader(&program, fragment_shader);
    CTX.context.borrow().context.link_program(&program);

    program
}
