mod render_error;
mod gl_window;
mod gl_handler;
mod camera;
mod ui_camera;
mod framebuffer;
mod gl_framebuffer_multisample;
mod initialise_attrib_pointers;
mod vertex_3d;
mod vertex_2d;
mod renderable;
mod bloom_framebuffer;
pub mod texture;
pub mod shader;
pub mod types;
pub mod math;

use mockall_double::double;

pub use gl_window::GLWindow;
pub use gl_handler::GLHandler;
pub use render_error::RenderError;
pub use camera::Camera;
pub use ui_camera::UICamera;
pub use framebuffer::FrameBuffer;
pub use gl_framebuffer_multisample::FrameBufferMultisample;
pub use initialise_attrib_pointers::Vertex;
pub use vertex_3d::Vertex3d;
pub use vertex_2d::Vertex2d;
pub use bloom_framebuffer::BloomFramebuffer;

#[double]
pub use renderable::Renderable;
