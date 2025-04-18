/// module for common code and utilities
pub mod common;

pub use common::err::*;

use common::log::initialize_logs;
use display::{shader::Program, vertex::{Buffer, Vertex, VertexArray}, win::{initialize_glfw, initialize_opengl, initialize_window, GlfwCreateWindowProps}};
use glfw::{Context, WindowMode};

/// module for rendering and windowing using 
/// opengl and glfw.
pub mod display;

const VERTEX_COUNT: usize = 4;
const VERTICES: [Vertex; VERTEX_COUNT] = [
    Vertex { position: [0.5, 0.5, 0.0] },
    Vertex { position: [0.5, -0.5, 0.0] },
    Vertex { position: [-0.5, -0.5, 0.0] },
    Vertex { position: [-0.5, 0.5, 0.0] }
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

    let mut vao = VertexArray::new(&gl);
    vao.bind(&gl);

    let mut vbo = Buffer::new_vertex(&gl);
    vbo.bind(&gl);
    vbo.data(&gl, &VERTICES[..]);

    let mut ebo = Buffer::new_element(&gl);
    ebo.bind(&gl);
    ebo.data(&gl, &INDICES[..]);

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

            let timeValue = glfw.get_time();
            let greenValue = (timeValue.sin() / 2.0) + 0.5;
            let greenValue = greenValue as f32;
            let vertexColorLocation = gl.GetUniformLocation(program.id, "ourColor".as_ptr());

            gl.Uniform3f(vertexColorLocation, 0.0, greenValue, 0.0);
            
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
