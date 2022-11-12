use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};

use crate::renderer::SHADERS;

pub fn compile_program(ctx: &WebGl2RenderingContext, name: &str) -> WebGlProgram {
    let fragment_shader = compile_shader(&ctx, WebGl2RenderingContext::FRAGMENT_SHADER, name)
        .expect("Compiling fragmet shader failed");

    let vertex_shader = compile_shader(&ctx, WebGl2RenderingContext::VERTEX_SHADER, name)
        .expect("Compiling vertex shader failed");

    link_program(&ctx, &vertex_shader, &fragment_shader)
}

fn compile_shader(
    ctx: &WebGl2RenderingContext,
    shader_type: u32,
    name: &str,
) -> Result<WebGlShader, String> {
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

    let shader = ctx
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    ctx.shader_source(&shader, shader_source);
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
