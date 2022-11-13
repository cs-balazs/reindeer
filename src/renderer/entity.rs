use super::{lib, types, vertex::Vertex, RendererBackend};

#[derive(Debug)]
pub struct Entity {
    pub vertices: Vec<Vertex>,
    pub vao: lib::Vao,
    pub shader: lib::shader::Shader,
}

impl Entity {
    pub fn new(vertices: Vec<Vertex>, shader: lib::shader::Shader, ctx: &lib::Context) -> Entity {
        let program = ctx.compile_program("basic");
        ctx.use_program(&program);

        let vao = ctx.create_vertex_array();
        ctx.bind_vertex_array(&vao);

        let buffer = ctx.create_buffer();
        ctx.bind_buffer(types::ARRAY_BUFFER, &buffer);
        ctx.buffer_data(
            &vertices
                .iter()
                .flat_map(|v| {
                    vec![
                        v.position[0],
                        v.position[1],
                        v.position[2],
                        v.color.unwrap()[0],
                        v.color.unwrap()[1],
                        v.color.unwrap()[2],
                    ]
                })
                .collect::<Vec<f32>>(),
            types::ARRAY_BUFFER,
            types::STATIC_DRAW,
        );

        let stride = {
            match &vertices.first() {
                Some(first) => match (first.color, first.normal) {
                    (Some(_), Some(_)) => 36,
                    (Some(_), None) => 24,
                    (None, Some(_)) => 24,
                    _ => 0,
                },
                None => panic!("Expected at least one vertex"),
            }
        };

        ctx.enable_vertex_attrib_array(0u32);
        ctx.vertex_attrib_pointer(0u32, 3, types::FLOAT, false, stride, 0);

        if let Some(_) = vertices.first().and_then(|f| f.color) {
            ctx.enable_vertex_attrib_array(1u32);
            ctx.vertex_attrib_pointer(1u32, 3, types::FLOAT, false, stride, 12);
        }

        Entity {
            vertices,
            vao,
            shader,
        }
    }

    // Do NOT commit this... Implement uniforms and pass the transformation matrices
    pub fn draw(&self, ctx: &lib::Context) {
        ctx.use_program(&self.shader.id);
        ctx.bind_vertex_array(&self.vao);
        ctx.draw_arrays(types::TRIANGLES, 0, self.vertices.len() as i32);
    }
}
