#![allow(dead_code)]

use std::collections::HashSet;

use glfw::{Context, Glfw, Key, Action, GlfwReceiver};
use crate::types::{Vec2, UVec2, uvec2, ivec2, IVec2, vec2};

use crate::RenderError;
type Result<T> = std::result::Result<T, RenderError>;

/*pub enum GLWindow {
    Fullscreen { glfw_window: glfw::Window, events: Receiver<(f64, glfw::WindowEvent)>, window_width: u32, window_height: u32, fullscreen_width: u32, fullscreen_height: u32, clear_colour: [f32; 3], mouse_delta: Vec2, keys_pressed: HashSet<glfw::Key> },
    Windowed { glfw_window: glfw::Window, events: Receiver<(f64, glfw::WindowEvent)>, window_width: u32, window_height: u32, clear_colour: [f32; 3], mouse_delta: Vec2, keys_pressed: HashSet<glfw::Key> },
}*/

pub struct GLWindow {
    glfw_window: glfw::PWindow,
    events: GlfwReceiver<(f64, glfw::WindowEvent)>,
    window_size: UVec2, 
    window_pos: IVec2,
    fullscreen_size: UVec2, 
    fullscreen_pos: IVec2,
    fullscreen: bool,
    has_resized_this_frame: bool,
    clear_colour: [f32; 3], 
    mouse_pos: Vec2, 
    mouse_pos_relative: Vec2,
    mouse_delta: Vec2,
    mouse_delta_relative: Vec2,
    keys_pressed: HashSet<Key>
}

impl GLWindow {
    pub fn new (glfw: &mut Glfw, window_name: &str, window_width: u32, window_height: u32) -> Result<GLWindow> {
        let (mut window, events) = 
            match glfw.create_window(window_width, window_height, window_name, glfw::WindowMode::Windowed) {//init glfw window using args given from function args
                Some(res) => res,
                None =>  return Err(
                    RenderError::WindowError { 
                        window_name: window_name.to_string(), 
                        error: format!("Window did not create (width: {}, height: {})", window_width, window_height)
                    }
                ),
            };
            
        window.make_current(); //set new window as the current GL focus window
        window.set_key_polling(true);

        window.set_cursor_pos_polling(true);
        window.set_scroll_polling(true);
        window.set_mouse_button_polling(true);
        window.set_focus_polling(true);

        window.set_framebuffer_size_polling(true);
        window.set_pos_polling(true);

        let (window_pos_x, window_pos_y) = window.get_pos(); 


        Ok(GLWindow{ 
            glfw_window: window, 
            events,
            window_size: uvec2(window_width, window_height),
            window_pos: ivec2(window_pos_x, window_pos_y),
            fullscreen_size: uvec2(0, 0),
            fullscreen_pos: ivec2(0, 0),
            fullscreen: false,
            has_resized_this_frame: true,
            clear_colour: [0.0, 0.0, 0.0], 
            mouse_pos: vec2(0.0, 0.0), 
            mouse_pos_relative: vec2(0.0, 0.0),
            mouse_delta: vec2(0.0, 0.0), 
            mouse_delta_relative: vec2(0.0, 0.0),
            keys_pressed: HashSet::new() 
        })
    }


    pub fn update_viewport(&self) {
        if self.fullscreen{
            unsafe { gl::Viewport(0, 0, self.fullscreen_size.x as i32, self.fullscreen_size.y as i32); } // set the viewport size
        }
        else {
            unsafe { gl::Viewport(0, 0, self.window_size.x as i32, self.window_size.y as i32); }
        }
    }

    
    pub fn handle_events (&mut self, glfw: &mut Glfw) -> Vec<glfw::WindowEvent>{
        let mut events_to_return: Vec<glfw::WindowEvent> = vec![];
        let messages = {
            let mut new_vec = Vec::new();
            for (_, event) in glfw::flush_messages(&self.events) {
                new_vec.push(event);
            }
            new_vec
        };

        for event in messages{ //handle gl events
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    if self.fullscreen { 
                        self.fullscreen_size = uvec2(width as u32, height as u32);
                    }
                    else {
                        self.window_size = uvec2(width as u32, height as u32);
                    }
                    self.update_viewport(); // change the window framebuffer size to new window/screen size
                    self.has_resized_this_frame = true; 
                }
                glfw::WindowEvent::Pos(x, y) => {
                    if !self.fullscreen {
                        self.window_pos = ivec2(x, y);
                    }

                }

                glfw::WindowEvent::CursorPos(xpos, ypos) => {
                    let current_size = if self.fullscreen { self.fullscreen_size } else { self.window_size };
                    self.mouse_delta = vec2(xpos as f32 - self.mouse_pos.x, ypos as f32 - self.mouse_pos.y);
                    self.mouse_pos = vec2(xpos as f32, ypos as f32);

                    self.mouse_pos_relative = vec2(
                        (2.0 * self.mouse_pos.x / current_size.x as f32) - 1.0,
                        (2.0 * self.mouse_pos.y / current_size.y as f32) - 1.0
                    );
                    self.mouse_delta_relative = vec2(
                        2.0 * self.mouse_delta.x / current_size.x as f32,
                        2.0 * self.mouse_delta.y / current_size.y as f32
                    );
                }
                glfw::WindowEvent::Key(Key::F11, _, Action::Press, _) => {
                    let mode = !self.fullscreen;
                    self.set_fullscreen_mode(mode, glfw);

                }
                glfw::WindowEvent::Key(key, _, Action::Press, _)  => {
                    if !self.keys_pressed.contains(&key) {
                        self.keys_pressed.insert(key);
                    }
                }
                glfw::WindowEvent::Key(key, _, Action::Release, _)  => {
                    if self.keys_pressed.contains(&key) {
                        self.keys_pressed.remove(&key);
                    }
                }
                        
                _ => {}
            }

            events_to_return.push(event); //store events just incase the program wants to respond to something outside this handler
        }
        events_to_return
        
    }

    pub fn set_fullscreen_mode (&mut self, fullscreen: bool, glfw: &mut Glfw) {

        if fullscreen && !self.fullscreen  {
            glfw.with_connected_monitors(|_, monitors| {
                let monitor = &monitors[0];

                let (fullscreen_pos_x, fullscreen_pos_y) = monitor.get_pos();
                let monitor_mode = monitor.get_video_mode().unwrap();

                self.fullscreen_size = uvec2(monitor_mode.width, monitor_mode.height);
                self.fullscreen_pos = ivec2(fullscreen_pos_x, fullscreen_pos_y);
                self.glfw_window.set_monitor(glfw::WindowMode::FullScreen(monitor), self.fullscreen_pos.x, self.fullscreen_pos.y, self.fullscreen_size.x, self.fullscreen_size.y, None);

            });}
        else if self.fullscreen {
            self.glfw_window.set_monitor(glfw::WindowMode::Windowed, self.window_pos.x, self.window_pos.y, self.window_size.x, self.window_size.y, None);
        }

        self.fullscreen = fullscreen;
        self.update_viewport();
    }

    pub fn reset_deltas(&mut self) {
        self.mouse_delta = vec2(0.0, 0.0);
        self.mouse_delta_relative = vec2(0.0, 0.0);
        self.has_resized_this_frame = false;
    }

    /* GETTERS AND SETTERS */

    pub fn get_glfw_window_mut(&mut self) -> &mut glfw::Window {
        &mut self.glfw_window
    }

    pub fn get_glfw_window(&self) -> &glfw::Window {
        &self.glfw_window
    }

    
    pub fn set_clear_color(&mut self, red: f32, green: f32, blue: f32) {
        unsafe { gl::ClearColor(red, green, blue, 1.0); }          // set default clear colour

        self.clear_colour = [red, green, blue];
    }
    
    pub fn set_title(&mut self, title: &str) {
        self.glfw_window.set_title(title);
    }


    pub fn has_resized_this_frame (&self) -> bool {
        self.has_resized_this_frame
    }

    pub fn get_window_size(&self) -> UVec2 {
        if self.fullscreen {
            self.fullscreen_size
        }
        else {
            self.window_size
        }
    }

    pub fn get_mouse_pos(&self) -> Vec2 {
        self.mouse_pos
    }

    pub fn get_mouse_pos_relative(&self) -> Vec2 {
        self.mouse_pos_relative
    }

    pub fn get_mouse_delta(&self) -> Vec2 {
        self.mouse_delta
    }

    pub fn get_mouse_delta_relative(&self) -> Vec2 {
        self.mouse_delta_relative
    }

    pub fn has_key_pressed(&mut self, key: glfw::Key) -> bool{
        self.keys_pressed.contains(&key)
    }
}