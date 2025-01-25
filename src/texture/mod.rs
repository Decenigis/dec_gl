mod gl_texture_2d;
mod gl_texture_2d_int;
mod gl_texture_1d;
mod gl_texture_1d_int;
mod texture_manager;
mod gl_texture_3d;

pub use gl_texture_2d::Texture2D;
pub use gl_texture_2d_int::Texture2DInt;
pub use gl_texture_1d::Texture1D;
pub use gl_texture_1d_int::Texture1DInt;
pub use texture_manager::TextureManager;
pub use gl_texture_3d::Texture3D;