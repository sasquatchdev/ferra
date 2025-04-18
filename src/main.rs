/// module for common code and utilities
pub mod common;

pub use common::err::*;

use common::log::initialize_logs;
use display::{shader::Program, win::{initialize_glfw, initialize_opengl, initialize_window, GlfwCreateWindowProps}};
use glfw::{Context, WindowMode};

/// module for rendering and windowing using 
/// opengl and glfw.
pub mod display;

const VERTEX_COUNT: usize = 4;
const VERTICES: [f32; VERTEX_COUNT * 3] = [
    0.5, 0.5, 0.0,
    0.5, -0.5, 0.0,
    -0.5, -0.5, 0.0,
    -0.5, 0.5, 0.0
];

const INDICES: [u32; 6] = [
    0, 1, 3,
    1, 2, 3
];

const VERT_SRC: &str = include_str!("../res/vertex.glsl");
const FRAG_SRC: &str = include_str!("../res/fragment.glsl");

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

    let mut vao = 0;
    unsafe {
        gl.GenVertexArrays(1, &mut vao);
        gl.BindVertexArray(vao);
    }

    let mut vbo = 0;
    unsafe {
        gl.GenBuffers(1, &mut vbo);
        gl.BindBuffer(gl33::GL_ARRAY_BUFFER, vbo);
        gl.BufferData(gl33::GL_ARRAY_BUFFER, size_of_val(&VERTICES) as isize, VERTICES.as_ptr().cast(), gl33::GL_STATIC_DRAW);
    }

    let mut ebo = 0;
    unsafe {
        gl.GenBuffers(1, &mut ebo);
        gl.BindBuffer(gl33::GL_ELEMENT_ARRAY_BUFFER, ebo);
        gl.BufferData(gl33::GL_ELEMENT_ARRAY_BUFFER, size_of_val(&INDICES) as isize, INDICES.as_ptr().cast(), gl33::GL_STATIC_DRAW);
    }

    let mut program = Program::new();
    program.link(&gl, VERT_SRC, FRAG_SRC)?;
    
    log::info!("Loaded GLSL shader program.");

    gl.UseProgram(program.id);

    unsafe {
        gl.VertexAttribPointer(0, 3, gl33::GL_FLOAT, 0, (3 * size_of::<f32>()) as i32, std::ptr::null());
        gl.EnableVertexAttribArray(0);
    }

    while !window.should_close() {
        unsafe {
            gl.UseProgram(program.id);

            gl.ClearColor(0.2, 0.3, 0.3, 1.0);
            gl.Clear(gl33::GL_COLOR_BUFFER_BIT);

            gl.BindVertexArray(vao);
            gl.DrawElements(gl33::GL_TRIANGLES, 6, gl33::GL_UNSIGNED_INT, std::ptr::null());
            gl.BindVertexArray(0);
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
