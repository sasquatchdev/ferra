use std::{cell::{Cell, RefCell}, ffi::CStr, rc::Rc};

use gl33::GlFns;
use glfw::{fail_on_errors, Context, Glfw, GlfwReceiver, PWindow, WindowEvent, WindowHint};
use crate::*;

pub const GLFW_VERSION_MAJOR: u32 = 3;
pub const GLFW_VERSION_MINOR: u32 = 3;

pub fn initialize_glfw() -> Result<Glfw> {
    log::debug!("Initializing GLFW...");
    let mut glfw = glfw::init(fail_on_errors!())?;
    log::info!("Initialized GLFW.");

    log::debug!("Setting GLFW error callback...");
    glfw.set_error_callback(callback);
    log::debug!("Set GLFW error callback.");

    let version = glfw::get_version();
    log::info!("GLFW version: {}.{}.{}", version.major, version.minor, version.patch);

    log::debug!("Setting GLFW window hints... (major = {}, minor = {}, profile = core)", GLFW_VERSION_MAJOR, GLFW_VERSION_MINOR);

    glfw.window_hint(WindowHint::ContextVersionMajor(GLFW_VERSION_MAJOR));
    glfw.window_hint(WindowHint::ContextVersionMinor(GLFW_VERSION_MINOR));
    glfw.window_hint(WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    log::debug!("Set GLFW window hints.");

    Ok(glfw)
}

pub struct GlfwCreateWindowProps {
    pub width: u32,
    pub height: u32,
    pub title: &'static str,
    pub mode: glfw::WindowMode<'static>,
}

pub fn initialize_window(glfw: &mut Glfw, props: GlfwCreateWindowProps) -> Result<(PWindow, GlfwReceiver<(f64, WindowEvent)>)> {
    log::debug!("Initializing GLFW window...");
    let window = glfw.create_window(props.width, props.height, props.title, props.mode);
    log::debug!("Initialized GLFW window.");

    if window.is_none() {
        log::error!("Failed to create GLFW window.");
        return Err(Error::GlfwWindowOrContext);
    }

    let (mut window, events) = window.unwrap();

    log::debug!("Setting GLFW window context...");
    window.make_current();
    log::debug!("Set GLFW window context.");

    Ok((window, events))
}

pub fn initialize_opengl(window: &mut PWindow) -> Result<GlFns> {
    log::debug!("Initializing OpenGL...");
    
    let window = Rc::new(RefCell::new(window));
    let count = Cell::new(0);

    log::debug!("Loading OpenGL functions...");
    let gl = unsafe {
        GlFns::load_from(&|s| {
            let name = CStr::from_ptr(s as *const i8);
            let name = name.to_str().unwrap();
            count.set(count.get() + 1);
            window.borrow_mut().get_proc_address(name)
        })
    };

    let gl = match gl {
        Ok(gl) => {
            log::info!("Loaded OpenGL functions: {}", count.get());
            gl
        },
        Err(err) => {
            log::error!("Failed to load OpenGL functions: {}", err);
            return Err(Error::GlLoad(err));
        }
    };

    log::debug!("Initialized OpenGL.");

    Ok(gl)
}

fn callback(_error: glfw::Error, description: String) {
    log::error!("GLFW Error: {}", description);
}
