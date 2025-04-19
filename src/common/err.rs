#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Glfw initialization failed. {0}")]
    GlfwInit(#[from] glfw::InitError),

    #[error("Glfw window or OpenGL context creation failed.")]
    GlfwWindowOrContext,

    #[error("Gl loading failed. {0}")]
    GlLoad(&'static str),

    #[error("Gl shader compilation failed. {0}")]
    GlShaderCompilation(String),

    #[error("Gl shader creation failed. {0}")]
    GlShaderCreation(String),

    #[error("Gl shader program creation failed. {0}")]
    GlShaderProgramCreation(String),

    #[error("Gl shader linking failed. {0}")]
    GlShaderProgramLinking(String),

    #[error("Gl uniform location not found. {0}")]
    GlUniformLocation(String),

    #[error("Image error. {0}")]
    Image(#[from] image::ImageError),
}

pub type Result<T> = std::result::Result<T, Error>;
