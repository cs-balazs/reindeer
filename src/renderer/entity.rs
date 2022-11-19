use super::{lib, types, vertex::Vertex, RendererBackend};

#[derive(Debug, Clone)]
pub struct Entity {
    pub vertices: Vec<Vertex>,
    pub vao: lib::Vao,
    pub shader: lib::shader::Shader,
    /*
    pub transformations: Vec<Transformation> -> Transformation::Scale(0.2) or Transformation::Transalte(0.2, 0.0, 0.0), ...
    */
}

impl Entity {
    pub fn new(vertices: Vec<Vertex>, shader: lib::shader::Shader, ctx: &lib::Context) -> Entity {
        let vao = ctx.create_vertex_array();
        ctx.bind_vertex_array(&vao);

        let buffer_data = vertices
            .iter()
            .flat_map(|v| {
                let mut vec = vec![v.position[0], v.position[1], v.position[2]];

                if let Some([r, g, b]) = v.color {
                    vec.push(r);
                    vec.push(g);
                    vec.push(b);
                }

                if let Some([x, y, z]) = v.normal {
                    vec.push(x);
                    vec.push(y);
                    vec.push(z);
                }

                vec
            })
            .collect::<Vec<f32>>();

        let buffer = ctx.create_buffer();
        ctx.bind_buffer(types::ARRAY_BUFFER, &buffer);
        ctx.buffer_data(&buffer_data, types::ARRAY_BUFFER, types::STATIC_DRAW);

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

        let color_offset = if vertices.first().and_then(|f| f.color).is_some() {
            12
        } else {
            0
        };

        if color_offset > 0 {
            ctx.enable_vertex_attrib_array(1u32);
            ctx.vertex_attrib_pointer(1u32, 3, types::FLOAT, false, stride, color_offset);
        }

        if vertices.first().and_then(|f| f.normal).is_some() {
            ctx.enable_vertex_attrib_array(2u32);
            ctx.vertex_attrib_pointer(2u32, 3, types::FLOAT, false, stride, 12 + color_offset);
        }

        Entity {
            vertices,
            vao,
            shader,
        }
    }

    pub fn draw(&self, ctx: &lib::Context) {
        ctx.use_program(&self.shader.id);
        ctx.bind_vertex_array(&self.vao);
        ctx.draw_arrays(types::TRIANGLES, 0, self.vertices.len() as i32);
    }
}
