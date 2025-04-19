/// module for common code and utilities
pub mod common;

pub use common::err::*;

use common::{log::initialize_logs, raw::AsRaw};
use display::{shader::Program, texture::Texture, vertex::{Buffer, Vertex, VertexArray}, win::{initialize_glfw, initialize_opengl, initialize_window, GlfwCreateWindowProps}};
use glfw::{Context, WindowMode};

/// module for rendering and windowing using 
/// opengl and glfw.
pub mod display;

const VERTEX_COUNT: usize = 4;
const VERTICES: [Vertex; VERTEX_COUNT] = [
    Vertex { position: [ 0.5, 0.5, 0.0 ], color: [ 1.0, 0.0, 0.0 ], texture: [1.0, 1.0] },
    Vertex { position: [ 0.5, -0.5, 0.0 ], color: [ 0.0, 1.0, 0.0 ], texture: [1.0, 0.0] },
    Vertex { position: [ -0.5, -0.5, 0.0 ], color: [ 0.0, 0.0, 1.0 ], texture: [0.0, 0.0] },
    Vertex { position: [ -0.5, 0.5, 0.0 ], color: [ 1.0, 1.0, 0.0 ], texture: [0.0, 1.0] },
];

const INDICES: [u32; 6] = [
    0, 1, 3,
    1, 2, 3,
];

const VERT_SRC: &str = include_str!("../res/shaders/vertex.glsl");
const FRAG_SRC: &str = include_str!("../res/shaders/fragment.glsl");

fn main() -> Result<()>
{
    #[cfg(debug_assertions)]
    unsafe {
        // set environment variables for debugging
        std::env::set_var("RUST_LOG", "debug");
        std::env::set_var("RUST_BACKTRACE", "1");
    }

    initialize_logs();

    let mut glfw = initialize_glfw()?;
    let (mut window, events) = initialize_window(&mut glfw, GlfwCreateWindowProps {
        width: 800,
        height: 600,
        title: "Hello World",
        mode: WindowMode::Windowed,
    })?;

    let gl = initialize_opengl(&mut window)?;

    let mut vao = VertexArray::new(&gl);
    let mut vbo = Buffer::new_vertex(&gl);
    let mut ebo = Buffer::new_element(&gl);

    vao.bind(&gl);

    vbo.bind(&gl);
    vbo.data(&gl, &VERTICES.as_raw());

    ebo.bind(&gl);
    ebo.data(&gl, &INDICES[..]);

    let mut program = Program::new();
    program.link(&gl, VERT_SRC, FRAG_SRC)?;
    
    log::info!("Loaded GLSL shader program.");

    unsafe {
        // Position attribute (3 floats starting at offset 0)
        gl.VertexAttribPointer(0, 3, gl33::GL_FLOAT, 0, (8 * size_of::<f32>()) as i32, std::ptr::null());
        gl.EnableVertexAttribArray(0);

        // Color attribute (3 floats starting at offset 3 * size_of::<f32>())
        gl.VertexAttribPointer(1, 3, gl33::GL_FLOAT, 0, (8 * size_of::<f32>()) as i32, (3 * size_of::<f32>()) as *const _);
        gl.EnableVertexAttribArray(1);

        // Texture coordinate attribute (2 floats starting at offset 6 * size_of::<f32>())
        gl.VertexAttribPointer(2, 2, gl33::GL_FLOAT, 0, (8 * size_of::<f32>()) as i32, (6 * size_of::<f32>()) as *const _);
        gl.EnableVertexAttribArray(2);
    }

    gl.UseProgram(program.id);

    let texture1 = Texture::load_file(&gl, "res/textures/container.jpg")?;
    let texture2 = Texture::load_file(&gl, "res/textures/awesomeface.png")?;

    texture1.uniform(&gl, &program, "texture1")?;
    texture2.uniform(&gl, &program, "texture2")?;

    while !window.should_close() {
        unsafe {
            gl.UseProgram(program.id);

            gl.ClearColor(0.2, 0.3, 0.3, 1.0);
            gl.Clear(gl33::GL_COLOR_BUFFER_BIT);

            texture1.activate(&gl)?;
            texture1.bind(&gl);

            texture2.activate(&gl)?;
            texture2.bind(&gl);

            vao.bind(&gl);
            gl.DrawElements(gl33::GL_TRIANGLES, 6, gl33::GL_UNSIGNED_INT, std::ptr::null());
            VertexArray::unbind(&gl);
        }
        
        window.swap_buffers();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Close => {
                    log::debug!("Window close event received.");
                    window.set_should_close(true);
                }
                _ => {
                    log::debug!("Glfw event received. {:?}", event);
                }
            }
        }
    }

    Ok(())    
}
