use std::{ffi::CString, marker::PhantomData};

use crate::{Error, Result};

pub struct Shader<T> {
    pub id: u32,

    /// the shader type
    shader_type: PhantomData<T>,
}

struct VertexShader;
struct FragmentShader;

pub struct Program {
    /// the internal open-gl program id
    pub id: u32,

    /// the vertex shader abstraction
    vertex: Shader<VertexShader>,

    /// the fragment shader abstraction
    fragment: Shader<FragmentShader>,
}

impl Program {
    fn link_error(
        &self,
        gl: &gl33::GlFns,
    ) -> Option<Error> {
        log::debug!("Checking for shader program linking errors...");

        let mut success = 0;
        let mut log_len = 0_i32;
        let mut v: Vec<u8> = vec![0; 1024];

        unsafe {
            gl.GetProgramiv(self.id, gl33::GL_LINK_STATUS, &mut success);
        }

        if success == 0 {
            log::debug!("Shader program linking failed. Retrieving error log...");
            unsafe {
                gl.GetProgramInfoLog(self.id, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
            }
            log::debug!("Shader program linking error: {}", String::from_utf8_lossy(&v));

            return Some(Error::GlShaderProgramLinking(
                String::from_utf8_lossy(&v).to_string(),
            ));
        }

        log::debug!("No program linking errors found.");
        None
    }

    pub fn new() -> Self {
        Self {
            id: 0,
            vertex: Shader::<VertexShader>::new(),
            fragment: Shader::<FragmentShader>::new(),
        }
    }

    pub fn link(
        &mut self,
        gl: &gl33::GlFns,
        vertex_source: &str,
        fragment_source: &str,
    ) -> Result<()> {
        log::debug!("Creating shader program...");

        self.id = gl.CreateProgram();

        if self.id == 0 {
            log::error!("Failed to create shader program");
            return Err(Error::GlShaderProgramCreation(
                "Failed to create shader program".to_string(),
            ));
        }

        log::debug!("Compiling shaders...");

        self.vertex.compile(gl, vertex_source)?;
        self.fragment.compile(gl, fragment_source)?;

        log::debug!("Attaching shaders...");

        gl.AttachShader(self.id, self.vertex.id);
        gl.AttachShader(self.id, self.fragment.id);

        log::debug!("Linking program...");

        gl.LinkProgram(self.id);

        if let Some(err) = self.link_error(gl) {
            log::error!("Shader program linking failed. {}", err);
            return Err(err);
        }

        log::debug!("Cleaning up...");

        gl.DeleteShader(self.vertex.id);
        gl.DeleteShader(self.fragment.id);

        log::debug!("Linked shader program successfully.");

        Ok(())
    }
}

impl<T> Shader<T> {
    pub fn new() -> Self {
        Self {
            id: 0,
            shader_type: PhantomData,
        }
    }

    fn compile_error(
        &self,
        gl: &gl33::GlFns,
    ) -> Option<Error> {
        log::debug!("Checking for shader compilation errors...");

        let mut success = 0;
        let mut log_len = 0_i32;
        let mut v: Vec<u8> = vec![0; 1024];

        unsafe {
            gl.GetShaderiv(self.id, gl33::GL_COMPILE_STATUS, &mut success);
        }

        if success == 0 {
            log::debug!("Shader compilation failed. Retrieving error log...");
            unsafe {
                gl.GetShaderInfoLog(self.id, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
            }

            log::debug!("Shader compilation error: {}", String::from_utf8_lossy(&v));

            return Some(Error::GlShaderCompilation(
                String::from_utf8_lossy(&v).to_string(),
            ));
        }

        log::debug!("No shader compilation errors found.");
        None
    }
}

impl Shader<VertexShader> {
    pub fn compile(
        &mut self,
        gl: &gl33::GlFns,
        source: &str,
    ) -> Result<()> {
        log::debug!("Compiling vertex shader...");

        let source = CString::new(source).unwrap();

        self.id = gl.CreateShader(gl33::GL_VERTEX_SHADER);

        if self.id == 0 {
            log::error!("Failed to create vertex shader");
            return Err(Error::GlShaderCreation(
                "Failed to create vertex shader".to_string(),
            ));
        }

        unsafe {
            gl.ShaderSource(self.id, 1, &source.as_ptr().cast(), std::ptr::null());
            gl.CompileShader(self.id);
        }

        if let Some(err) = self.compile_error(gl) {
            log::error!("Vertex shader compilation failed. {}", err);
            return Err(err);
        }

        log::debug!("Vertex shader compiled successfully. (id = {})", self.id);

        Ok(())
    }
}

impl Shader<FragmentShader> {
    pub fn compile(
        &mut self,
        gl: &gl33::GlFns,
        source: &str,
    ) -> Result<()> {
        log::debug!("Compiling fragment shader...");

        let source = CString::new(source).unwrap();

        self.id = gl.CreateShader(gl33::GL_FRAGMENT_SHADER);

        if self.id == 0 {
            log::error!("Failed to create fragment shader");
            return Err(Error::GlShaderCreation(
                "Failed to create fragment shader".to_string(),
            ));
        }

        unsafe {
            gl.ShaderSource(self.id, 1, &source.as_ptr().cast(), std::ptr::null());
            gl.CompileShader(self.id);
        }

        if let Some(err) = self.compile_error(gl) {
            log::error!("Fragment shader compilation failed. {}", err);
            return Err(err);
        }

        log::debug!("Fragment shader compiled successfully. (id = {})", self.id);

        Ok(())
    }
}
