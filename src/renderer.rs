use std::{cell::RefCell, collections::HashMap, str};

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
mod shader;
mod vertex;
mod vertex_attribute;

// ((?:-| )[0-9]\.[0-9]f32), ((?:-| )[0-9]\.[0-9]f32), ((?:-| )[0-9]\.[0-9]f32), ((?:-| )[0-9]\.[0-9]f32), ((?:-| )[0-9]\.[0-9]f32), ((?:-| )[0-9]\.[0-9]f32)
// Vertex { position: [$1, $2, $3], normal:  Some([$4, $5, $6]), color: None }

use entity::Entity;
use include_dir::{include_dir, Dir, DirEntry::File};
use scene::Scene;
use vertex::Vertex;
use vertex_attribute::VertexAttribute;

use crate::math::{get_rotation_matrix, get_scale_matrix, get_translation_matrix, mat4_mat4_mul};

use self::{lib::Shader, shader::Shader as S};

pub const WINDOW_WIDTH: u16 = 500;
pub const WINDOW_HEIGHT: u16 = 500;

#[cfg(feature = "webgl")]
const SHADERS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/assets/shaders/webgl");
#[cfg(feature = "opengl")]
const SHADERS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/assets/shaders/opengl");

// https://github.com/rustwasm/wasm-bindgen/issues/1505#issuecomment-489300331
struct WebGlContext {
    context: RefCell<lib::Context>,
}

unsafe impl Send for WebGlContext {}
unsafe impl Sync for WebGlContext {}

lazy_static! {
    static ref CTX: WebGlContext = WebGlContext {
        context: RefCell::from(lib::Context::new()),
    };
}

lazy_static! {
    pub static ref SHADERS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::<&str, &str>::new();

        SHADERS_DIR.entries().iter().for_each(|entry| {
            if let File(f) = entry {
                let name = f.path().to_str().unwrap();
                let content = f.contents_utf8().unwrap();
                map.insert(name.clone(), content.clone());
            } else {
                panic!("SHADERS initialization failed");
            }
        });

        map
    };
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

    fn draw_loop(draw_frame: impl FnMut() + 'static);
}

#[cfg_attr(feature = "webgl", wasm_bindgen(start))]
pub fn run() {
    CTX.context.borrow(); // To initialize OpenGL

    let light_position = [0.5, 0.5, -0.8];
    let light_color = [0.5, 0.5, 1.0];
    let object_color = [1.0, 0.5, 0.5];

    let shader_program = lib::shader::Shader::new("phong_light_object");
    shader_program.set_uniform("u_color", object_color);
    shader_program.set_uniform("u_light_color", light_color);
    shader_program.set_uniform("u_light_position", light_position);
    shader_program.set_uniform("u_camera_position", [0.0, 0.0, -1.0]);

    let shader_program_obj2 = lib::shader::Shader::new("uniform_color");
    shader_program_obj2.set_uniform("u_color", light_color);

    let vertices: Vec<[f32; 3]> = vec![
        [-0.5, -0.5, -0.5],
        [0.0, 0.0, -1.0],
        [0.5, -0.5, -0.5],
        [0.0, 0.0, -1.0],
        [0.5, 0.5, -0.5],
        [0.0, 0.0, -1.0],
        [0.5, 0.5, -0.5],
        [0.0, 0.0, -1.0],
        [-0.5, 0.5, -0.5],
        [0.0, 0.0, -1.0],
        [-0.5, -0.5, -0.5],
        [0.0, 0.0, -1.0],
        [-0.5, -0.5, 0.5],
        [0.0, 0.0, 1.0],
        [0.5, -0.5, 0.5],
        [0.0, 0.0, 1.0],
        [0.5, 0.5, 0.5],
        [0.0, 0.0, 1.0],
        [0.5, 0.5, 0.5],
        [0.0, 0.0, 1.0],
        [-0.5, 0.5, 0.5],
        [0.0, 0.0, 1.0],
        [-0.5, -0.5, 0.5],
        [0.0, 0.0, 1.0],
        [-0.5, 0.5, 0.5],
        [-1.0, 0.0, 0.0],
        [-0.5, 0.5, -0.5],
        [-1.0, 0.0, 0.0],
        [-0.5, -0.5, -0.5],
        [-1.0, 0.0, 0.0],
        [-0.5, -0.5, -0.5],
        [-1.0, 0.0, 0.0],
        [-0.5, -0.5, 0.5],
        [-1.0, 0.0, 0.0],
        [-0.5, 0.5, 0.5],
        [-1.0, 0.0, 0.0],
        [0.5, 0.5, 0.5],
        [1.0, 0.0, 0.0],
        [0.5, 0.5, -0.5],
        [1.0, 0.0, 0.0],
        [0.5, -0.5, -0.5],
        [1.0, 0.0, 0.0],
        [0.5, -0.5, -0.5],
        [1.0, 0.0, 0.0],
        [0.5, -0.5, 0.5],
        [1.0, 0.0, 0.0],
        [0.5, 0.5, 0.5],
        [1.0, 0.0, 0.0],
        [-0.5, -0.5, -0.5],
        [0.0, -1.0, 0.0],
        [0.5, -0.5, -0.5],
        [0.0, -1.0, 0.0],
        [0.5, -0.5, 0.5],
        [0.0, -1.0, 0.0],
        [0.5, -0.5, 0.5],
        [0.0, -1.0, 0.0],
        [-0.5, -0.5, 0.5],
        [0.0, -1.0, 0.0],
        [-0.5, -0.5, -0.5],
        [0.0, -1.0, 0.0],
        [-0.5, 0.5, -0.5],
        [0.0, 1.0, 0.0],
        [0.5, 0.5, -0.5],
        [0.0, 1.0, 0.0],
        [0.5, 0.5, 0.5],
        [0.0, 1.0, 0.0],
        [0.5, 0.5, 0.5],
        [0.0, 1.0, 0.0],
        [-0.5, 0.5, 0.5],
        [0.0, 1.0, 0.0],
        [-0.5, 0.5, -0.5],
        [0.0, 1.0, 0.0],
    ];

    let obj = Entity::new(
        vertices.clone(),
        Some(shader_program),
        Some(vec![
            Some(VertexAttribute::new(3, types::FLOAT, 4)),
            None,
            Some(VertexAttribute::new(3, types::FLOAT, 4)),
        ]),
    );
    let light_source = Entity::new(
        vertices.clone(),
        Some(shader_program_obj2),
        Some(vec![
            Some(VertexAttribute::new(3, types::FLOAT, 4)),
            None,
            Some(VertexAttribute::new(3, types::FLOAT, 4)),
        ]),
    );

    let scene = Scene::new(vec![obj, light_source]);
    CTX.context.borrow_mut().scenes.push(scene);

    CTX.context.borrow_mut().set_clear_color(1.0, 1.0, 1.0, 1.0);

    let rotation_angle = 0.002;
    let mut rotation_amount = 0.0;
    let mut light_source_rotation_amount = 42.42;

    let translation =
        get_translation_matrix(light_position[0], light_position[1], light_position[2]);
    let scale = get_scale_matrix(0.05, 0.05, 0.05);
    let light_source_model = mat4_mat4_mul(translation, scale);

    lib::Context::draw_loop(move || {
        CTX.context.borrow_mut().before_draw();

        let rotation = get_rotation_matrix(rotation_amount, rotation_amount, rotation_amount);
        let light_source_rotation = get_rotation_matrix(
            light_source_rotation_amount,
            light_source_rotation_amount,
            light_source_rotation_amount,
        );

        if let Some(current_scene) = CTX.context.borrow().scenes.first() {
            current_scene.draw(&CTX.context.borrow());

            current_scene.entities[0]
                .shader
                .as_ref()
                .unwrap()
                .set_uniform("u_model", rotation);

            current_scene.entities[1]
                .shader
                .as_ref()
                .unwrap()
                .set_uniform(
                    "u_model",
                    mat4_mat4_mul(light_source_model, light_source_rotation),
                );
        }

        rotation_amount += rotation_angle;
        light_source_rotation_amount += rotation_angle;

        CTX.context.borrow_mut().after_draw();
    });
}
