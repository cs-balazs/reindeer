#[cfg(not(feature = "opengl"))]
use web_sys::WebGl2RenderingContext;

pub mod buffer;
pub mod draw_mode;
pub mod shader;

#[cfg(not(feature = "opengl"))]
pub const FLOAT: u32 = WebGl2RenderingContext::FLOAT;
#[cfg(feature = "opengl")]
pub const FLOAT: u32 = gl::FLOAT;
