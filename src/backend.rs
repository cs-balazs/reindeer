pub trait Backend {
    type Context;
    type Shader;
    type Program;
    type Buffer;
    type Vao;

    fn compile_program(&self, name: &str) -> Self::Program;

    fn new() -> Self::Context;

    fn use_program(&self, program: &Self::Program);

    fn create_buffer(&self) -> Self::Buffer;

    fn bind_buffer(&self, buffer_type: u32, buffer: &Self::Buffer);

    fn buffer_data(&self, vertices: &[f32], buffer_type: u32, usage_hint: u32);

    fn create_vertex_array(&self) -> Self::Vao;

    fn bind_vertex_array(&self, vao: &Self::Vao);

    fn vertex_attrib_pointer(
        &self,
        index: u32,
        size: i32,
        type_: u32,
        normalized: bool,
        stride: i32,
        offset: i32,
    );

    fn enable_vertex_attrib_array(&self, index: u32);

    fn draw_arrays(&self, mode: u32, first: i32, vertex_count: i32);

    fn set_clear_color(&mut self, red: f32, green: f32, blue: f32, alpha: f32);

    fn clear(&self);

    fn should_close(&self) -> bool;

    fn before_draw(&mut self);

    fn after_draw(&mut self);

    fn draw_loop(draw_frame: impl FnMut() + 'static);
}
