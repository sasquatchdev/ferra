use std::marker::PhantomData;

use gl33::{GLenum, GlFns};

use crate::common::raw::AsRaw;

pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub texture: [f32; 2],
}

impl Vertex {
    pub fn new(position: [f32; 3], color: [f32; 3], texture: [f32; 2]) -> Self {
        Vertex { position, color, texture }
    }

    pub fn size() -> usize {
        std::mem::size_of::<Vertex>()
    }
}

impl AsRaw<f32> for Vertex {
    fn as_raw(self) -> Vec<f32> {
        let mut out = Vec::with_capacity(Vertex::size() / std::mem::size_of::<f32>());
        out.extend_from_slice(&self.position);
        out.extend_from_slice(&self.color);
        out.extend_from_slice(&self.texture);
        out
    }
}

pub struct VertexArray {
    pub id: u32
}

impl VertexArray {
    pub fn new(gl: &GlFns) -> Self {
        Self { id: unsafe {
            let mut id = 0;
            gl.GenVertexArrays(1, &mut id);
            id
        } }
    }

    pub fn bind(&mut self, gl: &GlFns) {
        gl.BindVertexArray(self.id);
    }

    pub fn unbind(gl: &GlFns) {
        gl.BindVertexArray(0);
    }
}

pub struct Buffer<T> {
    pub id: u32,
    pub type_: BufferType,
    pub data_: PhantomData<T>,
}

pub enum BufferType {
    Vertex,
    Element,
}

impl<T> Buffer<T> {
    fn target(&self) -> GLenum {
        match self.type_ {
            BufferType::Vertex => gl33::GL_ARRAY_BUFFER,
            BufferType::Element => gl33::GL_ELEMENT_ARRAY_BUFFER,
        }
    }

    pub fn new(gl: &GlFns, type_: BufferType) -> Self {
        Self { id: unsafe {
            let mut id = 0;
            gl.GenBuffers(1, &mut id);
            id
        }, type_, data_: PhantomData }
    }

    pub fn new_vertex(gl: &GlFns) -> Self {
        Self::new(gl, BufferType::Vertex)
    }

    pub fn new_element(gl: &GlFns) -> Self {
        Self::new(gl, BufferType::Element)
    }

    pub fn bind(&self, gl: &GlFns) {
        unsafe {
            gl.BindBuffer(self.target(), self.id);
        }
    }

    pub fn unbind(&self, gl: &GlFns) {
        unsafe {
            gl.BindBuffer(self.target(), 0);
        }
    }

    pub fn data(&mut self, gl: &GlFns, data: &[T]) {
        unsafe {
            gl.BufferData(
                self.target(),
                std::mem::size_of_val(data) as isize,
                data.as_ptr().cast(),
                gl33::GL_STATIC_DRAW,
            );
        }
    }
}
