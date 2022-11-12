#[cfg(feature = "webgl")]
pub mod webgl;
#[cfg(feature = "webgl")]
use wasm_bindgen::prelude::wasm_bindgen;
#[cfg(feature = "webgl")]
pub use web_sys::WebGl2RenderingContext as types;
#[cfg(feature = "webgl")]
pub use webgl as lib;

#[cfg(feature = "opengl")]
pub mod opengl;
#[cfg(feature = "opengl")]
use gl as types;
#[cfg(feature = "opengl")]
use glfw::Context;
#[cfg(feature = "opengl")]
pub use opengl as lib;

mod entity;
mod scene;
mod vertex;

use entity::Entity;
use scene::Scene;
use vertex::Vertex;

use crate::math::rotate;

pub const WINDOW_WIDTH: u16 = 500;
pub const WINDOW_HEIGHT: u16 = 500;

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

    fn draw_loop(draw_frame: impl FnMut() + 'static);
}

#[cfg_attr(feature = "webgl", wasm_bindgen(start))]
pub fn run() {
    let mut ctx = lib::Context::new();

    let program = ctx.compile_program("basic");

    let bottom = Vertex {
        position: [0.0, -0.8, 0.0],
        color: Some([1.0, 0.0, 0.0]),
        normal: None,
    };

    let top = Vertex {
        position: [0.0, 0.8, 0.0],
        color: Some([0.0, 1.0, 0.0]),
        normal: None,
    };

    let left = Vertex {
        position: [-0.8, 0.0, 0.0],
        color: Some([0.0, 0.0, 1.0]),
        normal: None,
    };

    let right = Vertex {
        position: [0.8, 0.0, 0.0],
        color: Some([0.5, 0.5, 0.0]),
        normal: None,
    };

    let forward = Vertex {
        position: [0.0, 0.0, 0.8],
        color: Some([0.0, 0.5, 0.5]),
        normal: None,
    };

    let behind = Vertex {
        position: [0.0, 0.0, -0.8],
        color: Some([0.5, 0.0, 0.5]),
        normal: None,
    };

    // Construct one scene with one entity
    let obj = Entity::new(
        vec![
            top.clone(),
            left.clone(),
            forward.clone(),
            top.clone(),
            forward.clone(),
            right.clone(),
            top.clone(),
            right.clone(),
            behind.clone(),
            top.clone(),
            behind.clone(),
            left.clone(),
            bottom.clone(),
            left.clone(),
            forward.clone(),
            bottom.clone(),
            forward.clone(),
            right.clone(),
            bottom.clone(),
            right.clone(),
            behind.clone(),
            bottom.clone(),
            behind.clone(),
            left.clone(),
        ],
        program,
        &ctx,
    );
    let scene = Scene::new(vec![obj]);
    ctx.scenes.push(scene);

    ctx.set_clear_color(1.0f32, 1.0f32, 0.0f32, 1.0f32);

    lib::Context::draw_loop(move || {
        ctx.before_draw();

        if let Some(ref mut current_scene) = &mut ctx.scenes.first_mut() {
            if let Some(ref mut first_entity) = &mut current_scene.entities.first_mut() {
                for vertex in &mut first_entity.vertices {
                    vertex.position = rotate(vertex.position, 0.002, 0.002, 0.002);
                }
            }
        }

        if let Some(current_scene) = ctx.scenes.first() {
            current_scene.draw(&ctx);
        }

        ctx.after_draw();
    });
}
