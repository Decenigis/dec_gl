mod texture_manager;
mod gl_texture_1d;
mod gl_texture_1d_int;
mod gl_texture_2d;
mod gl_texture_2d_int;
mod gl_texture_2d_u8;
mod gl_texture_3d;
mod gl_texture_3d_int;
mod gl_texture_3d_u8;

pub use texture_manager::TextureManager;
pub use gl_texture_1d::Texture1D;
pub use gl_texture_1d_int::Texture1DInt;
pub use gl_texture_2d::Texture2D;
pub use gl_texture_2d_int::Texture2DInt;
pub use gl_texture_2d_u8::Texture2Du8;
pub use gl_texture_3d::Texture3D;
pub use gl_texture_3d_int::Texture3DInt;
pub use gl_texture_3d_u8::Texture3Du8;

#[cfg(test)]
pub use gl_texture_1d::MockTexture1D;
#[cfg(test)]
pub use gl_texture_1d_int::MockTexture1DInt;
#[cfg(test)]
pub use gl_texture_2d::MockTexture2D;
#[cfg(test)]
pub use gl_texture_2d_int::MockTexture2DInt;
#[cfg(test)]
pub use gl_texture_2d_u8::MockTexture2Du8;
#[cfg(test)]
pub use gl_texture_3d::MockTexture3D;
#[cfg(test)]
pub use gl_texture_3d_int::MockTexture3DInt;
#[cfg(test)]
pub use gl_texture_3d_u8::MockTexture3Du8;
