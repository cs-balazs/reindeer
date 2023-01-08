use std::collections::HashSet;

use crate::{
    backend::Backend,
    lib::{shader::ShaderLib, Context},
};

pub type Vec3 = [f32; 3];

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
