use gl33::{GLenum, GlFns};
use image::ColorType;

use crate::{Error, Result};

use super::shader::Program;

pub struct Texture {
    pub id: u32,
    pub width: u32,
    pub height: u32,
    pub format: TextureFormat,
}

pub enum TextureFormat {
    Rgb,
    Rgba,
}

impl Texture {
    pub fn new(gl: &GlFns, width: u32, height: u32, format: TextureFormat) -> Self {
        Self { id: unsafe {
            let mut id = 0;
            gl.GenTextures(1, &mut id);
            id
        }, width, height, format }
    }

    pub fn unit(&self) -> u32 {
        self.id - 1
    }

    pub fn activate(&self, gl: &GlFns) -> Result<()> {
        unsafe {
            gl.ActiveTexture(match self.unit() {
                0..31 => GLenum(gl33::GL_TEXTURE0.0 + self.unit()),
                _ => Err(Error::GlTextureActivation(format!(
                    "Texture ID out of range: {}.",
                    self.id
                )))?,
            })
        }

        Ok(())
    }

    pub fn bind(&self, gl: &GlFns) {
        unsafe {
            gl.BindTexture(gl33::GL_TEXTURE_2D, self.id);
        }
    }

    pub fn unbind(gl: &GlFns) {
        unsafe {
            gl.BindTexture(gl33::GL_TEXTURE_2D, 0);
        }
    }

    pub fn data(&self, gl: &GlFns, data: &[u8]) {
        unsafe {
            gl.TexImage2D(
                gl33::GL_TEXTURE_2D,
                0,
                match self.format {
                    TextureFormat::Rgb => gl33::GL_RGB.0 as i32,
                    TextureFormat::Rgba => gl33::GL_RGBA.0 as i32,
                },
                self.width as i32,
                self.height as i32,
                0,
                match self.format {
                    TextureFormat::Rgb => gl33::GL_RGB,
                    TextureFormat::Rgba => gl33::GL_RGBA,
                },
                gl33::GL_UNSIGNED_BYTE,
                data.as_ptr() as *const _,
            );
        }
    }

    pub fn generate_mipmap(&self, gl: &GlFns) {
        unsafe {
            gl.GenerateMipmap(gl33::GL_TEXTURE_2D);
        }
    }

    pub fn load_data(gl: &GlFns, data: &[u8], width: u32, height: u32, format: TextureFormat) -> Self {
        let texture = Texture::new(gl, width, height, format);
        texture.bind(gl);
        texture.data(gl, data);
        texture.generate_mipmap(gl);
        texture
    }

    pub fn load_file(gl: &GlFns, path: &str) -> Result<Self> {
        log::debug!("Loading texture from file... {}", path);

        let img = image::open(path)?;
        let img = img.flipv();

        match img.color() {
            ColorType::Rgb8 => {
                let img = img.to_rgb8();
                let (width, height) = img.dimensions();
                let data = img.into_raw();
                let texture = Texture::load_data(gl, &data, width, height, TextureFormat::Rgb);
                Ok(texture)
            },
            ColorType::Rgba8 => {
                let img = img.to_rgba8();
                let (width, height) = img.dimensions();
                let data = img.into_raw();
                let texture = Texture::load_data(gl, &data, width, height, TextureFormat::Rgba);
                Ok(texture)
            },
            _ => {
                log::error!("Unsupported image format: {:?}", img.color());
                Err(Error::ImageFormat(format!(
                    "Unsupported image format: {:?}. Only RGB(8) and RGBA(8) are supported.",
                    img.color()
                )))
            }
        }
    }

    pub fn uniform(&self, gl: &GlFns, program: &Program, name: &str) -> Result<()> {
        log::debug!("Setting uniform {} to texture {}", name, self.unit());
        program.uniform_1i(gl, name, self.unit() as i32)
    }
}
