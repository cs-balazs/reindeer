use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};

// Can't read these from file, maybe try to load them in a hashmap in advance
const BASIC_FRAG: &str = r#"#version 300 es
precision highp float;
out vec4 outColor;

in vec4 frag_color;

void main() {
    outColor = frag_color;
}
"#;

const BASIC_VERT: &str = r#"#version 300 es
layout (location = 0) in vec3 position;
layout (location = 1) in vec3 vert_color;
        
out vec4 frag_color;

void main() {
    gl_Position = vec4(position, 1.0);
    frag_color = vec4(vert_color, 1.0);
}
"#;

pub fn compile_program(ctx: &WebGl2RenderingContext, name: &str) -> WebGlProgram {
    let fragment_shader = compile_shader(&ctx, WebGl2RenderingContext::FRAGMENT_SHADER, BASIC_FRAG)
        .expect("Compiling fragmet shader failed");

    let vertex_shader = compile_shader(&ctx, WebGl2RenderingContext::VERTEX_SHADER, BASIC_VERT)
        .expect("Compiling vertex shader failed");

    link_program(&ctx, &vertex_shader, &fragment_shader)
}

fn compile_shader(
    ctx: &WebGl2RenderingContext,
    shader_type: u32,
    file_path: &str,
) -> Result<WebGlShader, String> {
    let shader = ctx
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    ctx.shader_source(&shader, file_path);
    ctx.compile_shader(&shader);

    if ctx
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        if let Some(err) = ctx.get_shader_info_log(&shader) {
            let array = js_sys::Array::new();
            array.push(&err.into());
            web_sys::console::log(&array);
        }

        Err(ctx
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

fn link_program(
    ctx: &WebGl2RenderingContext,
    vertex_shader: &WebGlShader,
    fragment_shader: &WebGlShader,
) -> WebGlProgram {
    let program = ctx.create_program().unwrap();

    ctx.attach_shader(&program, vertex_shader);
    ctx.attach_shader(&program, fragment_shader);
    ctx.link_program(&program);

    program
}
