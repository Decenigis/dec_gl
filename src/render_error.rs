use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum RenderError {
    #[error("Error with OpenGL renderable object: {error}")]
    RenderableError { error: String },
    #[error("Error with OpenGL buffer object: {error}")]
    BufferError { error: String },
    #[error("Error with font object: {error}")]
    FontError { error: String },
    #[error("Failed to create integer texture: : {error}")]
    TextureIntError { error: String },
    #[error("Failed to load texture file at {texture_path}: {error}")]
    TextureError { texture_path: String, error: String },
    #[error("[{shader_name}({shader_type})] {error}")]
    ShaderError { shader_name: String, shader_type: String , error: String  },
    #[error("[{window_name}] {error}")]
    WindowError { window_name: String, error: String },
    #[error("{error}")]
    GLFWError { error: String }
}