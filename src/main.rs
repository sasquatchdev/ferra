/// module for common code and utilities
pub mod common;
pub use common::err::*;

use common::log::initialize_logs;
use display::win::{initialize_glfw, initialize_opengl, initialize_window, GlfwCreateWindowProps};
use glfw::{Context, WindowMode};

/// module for rendering and windowing using 
/// opengl and glfw.
pub mod display;

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

    let mut _gl = initialize_opengl(&mut window)?;

    while !window.should_close() {
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
