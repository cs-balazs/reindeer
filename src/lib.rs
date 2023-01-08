#[macro_use]
extern crate lazy_static;

pub mod backend;
pub mod common;
pub mod math;

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
// #[cfg(feature = "opengl")]
// use glfw::Context;
#[cfg(feature = "opengl")]
pub use opengl as lib;

mod entity;
mod scene;
mod vertex_attribute;

use crate::common::Shader;
use crate::{
    backend::Backend,
    common::Vec3,
    math::{get_rotation_matrix, get_scale_matrix, get_translation_matrix, mat4_mat4_mul},
};
use common::ShaderProgram;
use entity::Entity;
use include_dir::{include_dir, Dir, DirEntry::File};
use scene::Scene;
use std::{cell::RefCell, collections::HashMap, str};
use vertex_attribute::VertexAttribute;

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
                map.insert(<&str>::clone(&name), <&str>::clone(&content));
            } else {
                panic!("SHADERS initialization failed");
            }
        });

        map
    };
}

#[cfg_attr(feature = "webgl", wasm_bindgen(start))]
pub fn run() {
    CTX.context.borrow(); // To initialize OpenGL

    let light_position = [0.5, 0.5, -0.8];
    let light_color = [0.5, 0.5, 1.0];
    let object_color = [1.0, 0.5, 0.5];

    let shader_program = ShaderProgram::new("phong_light_object");
    shader_program.set_uniform("u_color", object_color);
    shader_program.set_uniform("u_light_color", light_color);
    shader_program.set_uniform("u_light_position", light_position);
    shader_program.set_uniform("u_camera_position", [0.0, 0.0, -1.0]);

    let shader_program_obj2 = ShaderProgram::new("uniform_color");
    shader_program_obj2.set_uniform("u_color", light_color);

    let vertices: Vec<Vec3> = vec![
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
        vertices,
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
