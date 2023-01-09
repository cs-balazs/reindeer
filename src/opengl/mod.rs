use super::{backend::Backend, WINDOW_HEIGHT, WINDOW_WIDTH};
use gl::types::{GLfloat, GLsizeiptr};
use glfw::{Context as GLFWContext, WindowEvent};
use std::mem;
use std::{ffi::c_void, sync::mpsc::Receiver};

pub mod shader;

pub struct Context {
    glfw: glfw::Glfw,
    window: glfw::Window,
    #[allow(unused)]
    events: Receiver<(f64, WindowEvent)>,
}

impl Backend for Context {
    type Context = Context;
    type Buffer = u32;
    type Program = u32;
    type Shader = u32;
    type Vao = u32;

    fn new() -> Self::Context {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Failed to initialize window.");
        #[cfg(target_os = "macos")]
        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 1));

        #[cfg(not(target_os = "macos"))]
        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));

        glfw.window_hint(glfw::WindowHint::Resizable(false));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        let (mut window, events) = glfw
            .create_window(
                WINDOW_WIDTH as u32,
                WINDOW_HEIGHT as u32,
                "reindeer",
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window.");

        window.set_key_polling(true);
        window.make_current();

        glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

        gl::load_with(|s| window.get_proc_address(s) as *const _);

        let (fb_width, fb_height) = window.get_framebuffer_size();

        unsafe {
            gl::Viewport(0, 0, fb_width, fb_height);
            gl::Enable(gl::DEPTH_TEST);
            // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        };

        Context {
            glfw,
            window,
            events,
        }
    }

    fn create_buffer(&self) -> Self::Buffer {
        let mut buf: u32 = 0;
        unsafe { gl::GenBuffers(1, &mut buf) }
        buf
    }

    fn bind_buffer(&self, buffer_type: u32, buffer: &Self::Buffer) {
        unsafe { gl::BindBuffer(buffer_type, *buffer) }
    }

    fn buffer_data(&self, vertices: &[f32], buffer_type: u32, usage_hint: u32) {
        unsafe {
            gl::BufferData(
                buffer_type,
                (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &vertices[0] as *const f32 as *const std::ffi::c_void,
                usage_hint,
            )
        }
    }

    fn bind_vertex_array(&self, vao: &Self::Vao) {
        unsafe { gl::BindVertexArray(*vao) }
    }

    fn clear(&self) {
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT) }
    }

    fn set_clear_color(&mut self, red: f32, green: f32, blue: f32, alpha: f32) {
        unsafe { gl::ClearColor(red, green, blue, alpha) }
    }

    fn create_vertex_array(&self) -> Self::Vao {
        let mut vao: u32 = 0;
        unsafe { gl::GenVertexArrays(1, &mut vao) }
        vao
    }

    fn draw_arrays(&self, mode: u32, first: i32, vertex_count: i32) {
        unsafe { gl::DrawArrays(mode, first, vertex_count) }
    }

    fn enable_vertex_attrib_array(&self, index: u32) {
        unsafe { gl::EnableVertexAttribArray(index) }
    }

    fn use_program(&self, program: &Self::Program) {
        unsafe { gl::UseProgram(*program) }
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
        unsafe {
            gl::VertexAttribPointer(
                index,
                size,
                type_,
                u8::from(normalized),
                stride,
                offset as *const c_void,
            )
        }
    }

    fn before_draw(&mut self) {
        self.clear();
        self.glfw.poll_events();
    }

    fn after_draw(&mut self) {
        self.window.swap_buffers();
    }

    fn draw_loop(mut draw_frame: impl FnMut() + 'static) {
        loop {
            draw_frame();
        }
    }
}
