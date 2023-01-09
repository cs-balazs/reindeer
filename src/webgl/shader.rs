use crate::{
    backend::Backend,
    common::{Shader, ShaderProgram, ShaderUtils},
    BACKEND, SHADERS,
};
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};

impl Shader<[f32; 3]> for ShaderProgram {
    fn set_uniform(&self, name: &str, input: [f32; 3]) {
        let loc = BACKEND
            .lock()
            .unwrap()
            .context
            .get_uniform_location(&self.id, name)
            .unwrap_or_else(|| panic!("Failed to find uniform location: {}", name));
        BACKEND.lock().unwrap().context.use_program(Some(&self.id));
        BACKEND
            .lock()
            .unwrap()
            .context
            .uniform3f(Some(&loc), input[0], input[1], input[2]);
    }
}

impl Shader<[[f32; 3]; 3]> for ShaderProgram {
    fn set_uniform(&self, name: &str, input: [[f32; 3]; 3]) {
        let loc = BACKEND
            .lock()
            .unwrap()
            .context
            .get_uniform_location(&self.id, name)
            .unwrap_or_else(|| panic!("Failed to find uniform location: {}", name));
        BACKEND.lock().unwrap().context.use_program(Some(&self.id));

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
        BACKEND
            .lock()
            .unwrap()
            .context
            .uniform_matrix3fv_with_f32_array(Some(&loc), false, &input);
    }
}

impl Shader<[[f32; 4]; 4]> for ShaderProgram {
    fn set_uniform(&self, name: &str, input: [[f32; 4]; 4]) {
        let loc = BACKEND
            .lock()
            .unwrap()
            .context
            .get_uniform_location(&self.id, name)
            .unwrap_or_else(|| panic!("Failed to find uniform location: {}", name));
        BACKEND.lock().unwrap().context.use_program(Some(&self.id));

        let input: [f32; 16] = [
            input[0][0],
            input[0][1],
            input[0][2],
            input[0][3],
            input[1][0],
            input[1][1],
            input[1][2],
            input[1][3],
            input[2][0],
            input[2][1],
            input[2][2],
            input[2][3],
            input[3][0],
            input[3][1],
            input[3][2],
            input[3][3],
        ];
        BACKEND
            .lock()
            .unwrap()
            .context
            .uniform_matrix4fv_with_f32_array(Some(&loc), false, &input);
    }
}

pub struct ShaderLib;

impl ShaderUtils for ShaderLib {
    fn compile_program(name: &str) -> <super::Context as Backend>::Program {
        let fragment_shader = compile_shader(WebGl2RenderingContext::FRAGMENT_SHADER, name)
            .expect("Compiling fragmet shader failed");

        let vertex_shader = compile_shader(WebGl2RenderingContext::VERTEX_SHADER, name)
            .expect("Compiling vertex shader failed");

        link_program(&vertex_shader, &fragment_shader)
    }
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

    let shader = BACKEND
        .lock()
        .unwrap()
        .context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    BACKEND
        .lock()
        .unwrap()
        .context
        .shader_source(&shader, shader_source);
    BACKEND.lock().unwrap().context.compile_shader(&shader);

    if BACKEND
        .lock()
        .unwrap()
        .context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        if let Some(err) = BACKEND.lock().unwrap().context.get_shader_info_log(&shader) {
            let array = js_sys::Array::new();
            array.push(&err.into());
            web_sys::console::log(&array);
        }

        Err(BACKEND
            .lock()
            .unwrap()
            .context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

fn link_program(vertex_shader: &WebGlShader, fragment_shader: &WebGlShader) -> WebGlProgram {
    let program = BACKEND.lock().unwrap().context.create_program().unwrap();

    BACKEND
        .lock()
        .unwrap()
        .context
        .attach_shader(&program, vertex_shader);
    BACKEND
        .lock()
        .unwrap()
        .context
        .attach_shader(&program, fragment_shader);
    BACKEND.lock().unwrap().context.link_program(&program);

    program
}
