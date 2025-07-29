extern crate gl;

use std::cell::RefCell;
use std::rc::Rc;
#[allow(unused_imports)]
use glfw::{Action, Context, Key, ffi::glfwSetInputMode};

use crate::{GLWindow, RenderError};
type Result<T> = std::result::Result<T, RenderError>;


pub struct GLHandler {
    glfw_instance: glfw::Glfw,
    glfw_window: GLWindow,

    vsync: bool,
}

impl GLHandler {
    pub fn new (window_name: &str, window_width: u32, window_height: u32, fullscreen: bool, vsync: bool) -> Result<Rc<RefCell<GLHandler>>>{
        let mut glfw = match glfw::init(glfw::fail_on_errors) { // Initialise GLFW, throw an error if that failed
            Ok(instance) => instance,
            Err(e) =>  return Err(RenderError::GLFWError { error: format!("GLFW init failed: {}", e) }),
        };

        glfw::WindowHint::ContextVersionMajor(3);
        glfw::WindowHint::ContextVersionMinor(3); // set the version of OpenGL to use
        glfw::WindowHint::OpenGlForwardCompat(true); //disable depricated functions
        glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core);


        glfw::WindowHint::Samples(Some(2));


        let mut window = match GLWindow::new(&mut glfw, window_name, window_width, window_height) {
            Ok(wind) => wind,
            Err(e) => return Err(e), //pass the error straight back to the app as this is unrecoverable
        }; //init window,

        gl::load_with(|f_name| window.get_glfw_window_mut().get_proc_address(f_name)); //load GL instructions with GLFW

        glfw.set_swap_interval(glfw::SwapInterval::Sync(if vsync {1} else {0}));

        window.set_fullscreen_mode(fullscreen, &mut glfw);

        window.update_viewport();
        window.set_clear_color(0.1, 0.2, 0.4);

        unsafe {
            gl::Enable(gl::LINE_SMOOTH);
            gl::Enable(gl::MULTISAMPLE);  //try and reduce the horrible jaggedies
        }

        Ok (Rc::new(RefCell::new(GLHandler {
            glfw_instance: glfw,
            glfw_window: window,

            vsync,
        })))
    }


    pub fn wind_should_close (&self) -> bool {
        self.glfw_window.get_glfw_window().should_close()
    }

    pub fn poll_window (&mut self) {
        self.glfw_window.get_glfw_window_mut().swap_buffers();
        self.glfw_instance.poll_events();
    }

    pub fn handle_events (&mut self) -> Vec<glfw::WindowEvent>{
        self.glfw_window.reset_deltas();
        self.glfw_window.handle_events(&mut self.glfw_instance)
    }

    pub fn set_vsync(&mut self, vsync: bool) {
        self.vsync = vsync;
        self.glfw_instance.set_swap_interval(glfw::SwapInterval::Sync(if vsync {1} else {0}));
    }

    pub fn get_vsync(&self) -> bool {
        self.vsync
    }

    /* GETTERS AND SETTERS */

    pub fn get_window (&self) -> &GLWindow {
        &self.glfw_window
    }
}
