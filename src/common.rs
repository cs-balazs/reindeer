use crate::{
    backend::Backend,
    lib::{shader::ShaderLib, Context},
};
use std::collections::HashSet;

pub type Vec3 = [f32; 3];

// ---- Shaders ----

pub trait ShaderUtils {
    fn compile_program(name: &str) -> <Context as Backend>::Program;
}

#[derive(Debug, Clone)]
pub struct ShaderProgram {
    pub id: <Context as Backend>::Program,
    pub uniforms: HashSet<String>,
}

impl ShaderProgram {
    pub fn new(name: &str) -> Self {
        let id = ShaderLib::compile_program(name);

        // TODO: Parse uniform names in self.uniforms

        Self {
            id,
            uniforms: HashSet::new(),
        }
    }
}

pub trait Shader<Input> {
    fn set_uniform(&self, name: &str, input: Input);
}

// ---- Vertex attributes ----

#[derive(Debug, Clone)]
pub struct VertexAttribute {
    pub count: u32,
    pub item_type: u32,
    pub item_size: u32,
}

impl VertexAttribute {
    pub fn new(count: u32, item_type: u32, item_size: u32) -> VertexAttribute {
        VertexAttribute {
            count,
            item_size,
            item_type,
        }
    }
}

pub const VERTEX_ATTRIBUTE_FVEC3: VertexAttribute = VertexAttribute {
    count: 3,
    item_type: crate::types::FLOAT,
    item_size: 4,
};
