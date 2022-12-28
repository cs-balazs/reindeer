pub mod shader;

use super::scene::Scene;
use super::{backend::Backend, WINDOW_HEIGHT, WINDOW_WIDTH};
use std::{cell::RefCell, rc::Rc, vec};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{
    window, WebGl2RenderingContext, WebGlBuffer, WebGlProgram, WebGlShader, WebGlVertexArrayObject,
    Window,
};

pub struct Context {
    context: WebGl2RenderingContext,
    #[allow(unused)]
    window: Window,

    pub scenes: Vec<Scene>,
}

impl Backend for Context {
    type Buffer = WebGlBuffer;
    type Context = Context;
    type Program = WebGlProgram;
    type Shader = WebGlShader;
    type Vao = WebGlVertexArrayObject;

    fn new() -> Context {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.query_selector("body").unwrap().unwrap();
        let canvas = document.create_element("canvas").unwrap();
        canvas
            .set_attribute("width", &WINDOW_WIDTH.to_string())
            .unwrap();
        canvas
            .set_attribute("height", &WINDOW_HEIGHT.to_string())
            .unwrap();
        body.append_child(&canvas).unwrap();
        let canvas: web_sys::HtmlCanvasElement =
            canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

        let context = canvas
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()
            .unwrap();

        context.enable(WebGl2RenderingContext::DEPTH_TEST);

        Context {
            context,
            window,
            scenes: vec![],
        }
    }

    fn compile_program(&self, name: &str) -> Self::Program {
        shader::compile_program(name)
    }

    fn use_program(&self, program: &WebGlProgram) {
        self.context.use_program(Some(program))
    }

    fn create_buffer(&self) -> WebGlBuffer {
        self.context.create_buffer().unwrap()
    }

    fn bind_buffer(&self, buffer_type: u32, buffer: &WebGlBuffer) {
        self.context.bind_buffer(buffer_type, Some(buffer));
    }

    fn buffer_data(&self, vertices: &[f32], buffer_type: u32, usage_hint: u32) {
        unsafe {
            let positions_array_buf_view = js_sys::Float32Array::view(vertices);

            self.context.buffer_data_with_array_buffer_view(
                buffer_type,
                &positions_array_buf_view,
                usage_hint,
            );
        }
    }

    fn create_vertex_array(&self) -> WebGlVertexArrayObject {
        self.context.create_vertex_array().unwrap()
    }

    fn bind_vertex_array(&self, vao: &WebGlVertexArrayObject) {
        self.context.bind_vertex_array(Some(vao))
    }

    fn vertex_attrib_pointer(
        &self,
        index: u32,
        size: i32,
        type_: u32,
        normalized: bool,
        stride: i32,
        offset: i32,
    ) {
        self.context
            .vertex_attrib_pointer_with_i32(index, size, type_, normalized, stride, offset);
    }

    fn enable_vertex_attrib_array(&self, index: u32) {
        self.context.enable_vertex_attrib_array(index);
    }

    fn draw_arrays(&self, mode: u32, first: i32, vertex_count: i32) {
        self.context.draw_arrays(mode, first, vertex_count);
    }

    fn set_clear_color(&mut self, red: f32, green: f32, blue: f32, alpha: f32) {
        self.context.clear_color(red, green, blue, alpha);
    }

    fn clear(&self) {
        self.context.clear(
            WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT,
        )
    }

    fn should_close(&self) -> bool {
        false
    }

    fn before_draw(&mut self) {
        self.clear();
    }

    fn after_draw(&mut self) {
        // sleep(time::Duration::from_millis(5))
    }

    fn draw_loop(mut draw_frame: impl FnMut() + 'static) {
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            draw_frame();
            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));

        request_animation_frame(g.borrow().as_ref().unwrap());
    }
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) -> i32 {
    window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK")
}
