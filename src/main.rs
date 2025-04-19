/// module for common code and utilities
pub mod common;

pub use common::err::*;

use common::{log::initialize_logs, raw::AsRaw};
use display::{shader::Program, vertex::{Buffer, Vertex, VertexArray}, win::{initialize_glfw, initialize_opengl, initialize_window, GlfwCreateWindowProps}};
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

    let image1 = image::open("res/textures/container.jpg").unwrap();
    let image1 = image1.flipv();
    let image1 = image1.to_rgb8();
    let (width1, height1) = image1.dimensions();
    let data1 = image1.into_raw();

    let mut texture1 = 0;
    unsafe {
        gl.GenTextures(1, &mut texture1);
        gl.BindTexture(gl33::GL_TEXTURE_2D, texture1);
        gl.TexImage2D(gl33::GL_TEXTURE_2D, 0, gl33::GL_RGB.0 as i32, width1 as i32, height1 as i32, 0, gl33::GL_RGB, gl33::GL_UNSIGNED_BYTE, data1.as_ptr() as *const _);
        gl.GenerateMipmap(gl33::GL_TEXTURE_2D);
    }

    let image2 = image::open("res/textures/awesomeface.png").unwrap();
    let image2 = image2.flipv();
    let image2 = image2.to_rgba8();
    let (width2, height2) = image2.dimensions();
    let data2 = image2.into_raw();

    let mut texture2 = 0;
    unsafe {
        gl.GenTextures(1, &mut texture2);
        gl.BindTexture(gl33::GL_TEXTURE_2D, texture2);
        gl.TexImage2D(gl33::GL_TEXTURE_2D, 0, gl33::GL_RGB.0 as i32, width2 as i32, height2 as i32, 0, gl33::GL_RGBA, gl33::GL_UNSIGNED_BYTE, data2.as_ptr() as *const _);
        gl.GenerateMipmap(gl33::GL_TEXTURE_2D);
    }

    program.uniform_1i(&gl, "texture1", 0)?;
    program.uniform_1i(&gl, "texture2", 1)?;

    while !window.should_close() {
        unsafe {
            gl.UseProgram(program.id);

            gl.ClearColor(0.2, 0.3, 0.3, 1.0);
            gl.Clear(gl33::GL_COLOR_BUFFER_BIT);

            gl.ActiveTexture(gl33::GL_TEXTURE0);
            gl.BindTexture(gl33::GL_TEXTURE_2D, texture1);

            gl.ActiveTexture(gl33::GL_TEXTURE1);
            gl.BindTexture(gl33::GL_TEXTURE_2D, texture2);

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
