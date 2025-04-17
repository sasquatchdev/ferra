#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Glfw initialization failed. {0}")]
    GlfwInit(#[from] glfw::InitError),

    #[error("Glfw window or OpenGL context creation failed.")]
    GlfwWindowOrContext,

    #[error("Gl loading failed. {0}")]
    GlLoad(&'static str),
}

pub type Result<T> = std::result::Result<T, Error>;
