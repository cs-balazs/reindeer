pub trait Shader<Input> {
    fn set_uniform(&self, name: &str, input: Input);
}
