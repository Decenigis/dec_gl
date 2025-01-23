pub mod gl_texture_2d;
pub mod gl_texture_2d_int;
pub mod gl_texture_1d;
pub mod gl_texture_1d_int;
pub mod texture_manager;


pub use gl_texture_2d::Texture2D;
pub use gl_texture_2d_int::Texture2DInt;
pub use gl_texture_1d::Texture1D;
pub use gl_texture_1d_int::Texture1DInt;
pub use texture_manager::TextureManager;