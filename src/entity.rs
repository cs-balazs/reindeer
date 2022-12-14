use super::{backend::Backend, lib, types};
use crate::{
    common::{ShaderProgram, Vec3, VertexAttribute},
    BACKEND,
};

#[derive(Debug, Clone)]
pub struct Entity {
    pub vertices: Vec<f32>,
    pub vao: <lib::Context as Backend>::Vao,
    pub shader: Option<ShaderProgram>,
    /*
    pub transformations: Vec<Transformation> -> Transformation::Scale(0.2) or Transformation::Transalte(0.2, 0.0, 0.0), ...
    */
    pub attributes: Option<Vec<Option<VertexAttribute>>>,
}

impl Entity {
    pub fn new(
        // ctx: &lib::Context,
        vertices: Vec<Vec3>,
        shader: Option<ShaderProgram>,
        vertex_attributes: Option<Vec<Option<VertexAttribute>>>,
    ) -> Entity {
        let vao = BACKEND.lock().unwrap().create_vertex_array();
        BACKEND.lock().unwrap().bind_vertex_array(&vao);

        let data = vertices.into_iter().flatten().collect::<Vec<f32>>();

        let buffer = BACKEND.lock().unwrap().create_buffer();
        BACKEND
            .lock()
            .unwrap()
            .bind_buffer(types::ARRAY_BUFFER, &buffer);
        BACKEND
            .lock()
            .unwrap()
            .buffer_data(&data, types::ARRAY_BUFFER, types::STATIC_DRAW);

        if let Some(attributes) = &vertex_attributes {
            let stride = attributes
                .iter()
                .map(|opt| {
                    if opt.is_some() {
                        (opt.as_ref().unwrap().count * opt.as_ref().unwrap().item_size)
                            .try_into()
                            .expect("Calculating stride failed. Cast to i32 failed.")
                    } else {
                        0
                    }
                })
                .sum();

            let mut offset = 0;

            for (index, attribute_options) in attributes.iter().enumerate() {
                match attribute_options {
                    Some(attribute) => {
                        let ind = index
                            .try_into()
                            .expect("Vertex attribute index cast to u32 failed");

                        BACKEND.lock().unwrap().enable_vertex_attrib_array(ind);
                        BACKEND.lock().unwrap().vertex_attrib_pointer(
                            ind,
                            attribute
                                .count
                                .try_into()
                                .expect("Vertex attribute count cast to i32 failed"),
                            attribute.item_type,
                            false, // TODO
                            stride,
                            offset,
                        );

                        let offset_increment: i32 =
                            (attribute.count * attribute.item_size).try_into().expect(
                                "Failed to increment vertex attribute offset. Cast to i32 failed.",
                            );

                        offset += offset_increment;
                    }
                    None => (),
                }
            }
        }

        Entity {
            vertices: data,
            vao,
            shader,
            attributes: vertex_attributes,
        }
    }

    pub fn draw(&self, ctx: &lib::Context) {
        if let Some(shader) = &self.shader {
            ctx.use_program(&shader.id);
        }
        ctx.bind_vertex_array(&self.vao);

        let divide_vertex_count_by: usize = self
            .attributes
            .as_ref()
            .and_then(|a| {
                a.iter().fold(Some(0), |acc, item| {
                    Some(acc.unwrap() + item.as_ref().map(|attribute| attribute.count).unwrap_or(0))
                })
            })
            .unwrap_or(1)
            .try_into()
            .unwrap();

        ctx.draw_arrays(
            types::TRIANGLES,
            0,
            (self.vertices.len() / divide_vertex_count_by)
                .try_into()
                .unwrap(),
        );
    }

    pub fn bind_shader(&mut self, shader: ShaderProgram) {
        self.shader = Some(shader);
    }

    pub fn push_vertex_attribute(&mut self, _attribute: VertexAttribute) {}
}
