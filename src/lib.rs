#[macro_use]
extern crate lazy_static;

pub mod backend;
pub mod common;
pub mod math;

#[cfg(feature = "webgl")]
pub mod webgl;
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

pub mod entity;
pub mod scene;

use crate::backend::Backend;
pub use common::{ShaderProgram, VERTEX_ATTRIBUTE_FVEC3};
use include_dir::{include_dir, Dir, DirEntry::File};
use std::sync::Mutex;
use std::{collections::HashMap, str};

pub const WINDOW_WIDTH: u16 = 500;
pub const WINDOW_HEIGHT: u16 = 500;

#[cfg(feature = "webgl")]
const SHADERS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/assets/shaders/webgl");
#[cfg(feature = "opengl")]
const SHADERS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/assets/shaders/opengl");

use once_cell::sync::Lazy;
use send_wrapper::SendWrapper;

static BACKEND: Mutex<Lazy<SendWrapper<lib::Context>>> =
    Mutex::new(Lazy::new(|| SendWrapper::new(lib::Context::new())));

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

pub fn init() {
    BACKEND.lock().unwrap().valid();
}

pub fn set_clear_color(red: f32, green: f32, blue: f32, alpha: f32) {
    BACKEND
        .lock()
        .unwrap()
        .set_clear_color(red, green, blue, alpha)
}
