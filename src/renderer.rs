#[cfg(feature = "webgl")]
mod webgl;
#[cfg(feature = "webgl")]
use wasm_bindgen::prelude::wasm_bindgen;
#[cfg(feature = "webgl")]
use web_sys::WebGl2RenderingContext as types;
#[cfg(feature = "webgl")]
use webgl as lib;

#[cfg(feature = "opengl")]
mod opengl;
#[cfg(feature = "opengl")]
use gl as types;
#[cfg(feature = "opengl")]
use glfw::Context;
#[cfg(feature = "opengl")]
use opengl as lib;

pub const WINDOW_WIDTH: u16 = 500;
pub const WINDOW_HEIGHT: u16 = 500;

#[derive(Debug)]
pub struct Vertex {
    position: [f32; 3],
    color: Option<[f32; 3]>,
    normal: Option<[f32; 3]>,
}

#[derive(Debug)]
pub struct Entity {
    vertices: Vec<Vertex>,
    vao: lib::Vao,
    shader: lib::Program,
}

pub trait Drawable {
    fn draw(&self, ctx: &lib::Context);
}

pub struct Scene<'a> {
    entities: Vec<Box<&'a dyn Drawable>>,
}

impl Entity {
    fn new(vertices: Vec<Vertex>, shader: lib::Program, ctx: &lib::Context) -> Entity {
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

        ctx.enable_vertex_attrib_array(0u32);
        ctx.vertex_attrib_pointer(0u32, 3, types::FLOAT, false, 24, 0);

        ctx.enable_vertex_attrib_array(1u32);
        ctx.vertex_attrib_pointer(1u32, 3, types::FLOAT, false, 24, 12);

        Entity {
            vertices,
            vao,
            shader,
        }
    }
}

impl<'a> Scene<'a> {
    fn new(entities: Vec<Box<&'a dyn Drawable>>) -> Scene<'a> {
        Scene { entities }
    }
}

impl Drawable for Entity {
    fn draw(&self, ctx: &lib::Context) {
        ctx.bind_vertex_array(&self.vao);
        ctx.draw_arrays(types::TRIANGLES, 0, self.vertices.len() as i32);
    }
}

impl<'a> Drawable for Scene<'a> {
    fn draw(&self, ctx: &lib::Context) {
        for entity in &self.entities {
            entity.draw(ctx);
        }
    }
}

pub trait RendererBackend {
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

    fn buffer_data(&self, vertices: &Vec<f32>, buffer_type: u32, usage_hint: u32);

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
}

#[cfg_attr(feature = "webgl", wasm_bindgen(start))]
pub fn run() {
    let mut ctx = lib::Context::new();

    let program = ctx.compile_program("basic");

    // Construct one scene with one entity
    let obj = Entity::new(
        vec![
            Vertex {
                position: [-0.7, -0.7, 0.0],
                color: Some([1.0, 0.0, 0.0]),
                normal: None,
            },
            Vertex {
                position: [0.7, -0.7, 0.0],
                color: Some([0.0, 1.0, 0.0]),
                normal: None,
            },
            Vertex {
                position: [0.0, 0.7, 0.0],
                color: Some([0.0, 0.0, 1.0]),
                normal: None,
            },
        ],
        program,
        &ctx,
    );
    let scene = Scene::new(vec![Box::new(&obj)]);
    ctx.scenes.push(scene);

    // Draw loop
    ctx.set_clear_color(1.0f32, 1.0f32, 0.0f32, 1.0f32);

    while !ctx.should_close() {
        ctx.before_draw();

        if let Some(current_scene) = ctx.scenes.first() {
            current_scene.draw(&ctx);
        }

        ctx.after_draw();
    }
}
