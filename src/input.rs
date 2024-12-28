use std::collections::HashMap;
use glium::winit::event::ElementState;
use glium::winit::keyboard::KeyCode;
use glium::winit::event::WindowEvent;
use glium::winit::event::WindowEvent::CursorMoved;
use glium::winit::event::WindowEvent::MouseInput;
use glium::winit::event::MouseButton::*;

// this code was stolen from https://github.com/rust-windowing/glutin/issues/708 because i couldn't
// be asked to write it myself
/// Keeps track of which keys have been pressed.
/// I have further edited it to keep track of the current mouse position
/// potential BUG, the program doesn't know the mouse position until the user has moved the cursor
#[derive(Clone)]
pub struct KeyboardState {
    state: HashMap<KeyCode, ElementState>,
}

#[derive(Clone)]
pub struct MouseState {
    pub pos : (f64, f64),
    pub left_button_pressed : bool,
    pub right_button_pressed : bool,
}

impl MouseState {
    pub fn new() -> MouseState {
        MouseState {
            pos : (0f64, 0f64),
            left_button_pressed : false,
            right_button_pressed : false,
        }
    }
    
    pub fn process_event(&mut self, window_event: &WindowEvent) {
        self.left_button_pressed = false;
        self.right_button_pressed = false;
        match window_event {
            CursorMoved {position : pos, ..} => {
                self.pos = (pos.x, pos.y);
            },
            MouseInput {button, ..} => {
                match button {
                    Left => self.left_button_pressed = true,
                    Right => self.right_button_pressed = true,
                    _ => (),
                }
            }
            _ => (),
        }
    }
}

impl KeyboardState {
    /// Constructs a new KeyboardState with all the keys released.
    pub fn new() -> KeyboardState {
        KeyboardState {
            state: HashMap::new(),
        }
    }

    /// Returns true if `key` is pressed.
    pub fn is_pressed(&self, key: KeyCode) -> bool {
        self.state
            .get(&key)
            .map(|&s| s == ElementState::Pressed)
            .unwrap_or(false)
    }

    /// Returns true if `key` is released.
    #[allow(dead_code)]
    pub fn is_released(&self, key: KeyCode) -> bool {
        !self.is_pressed(key)
    }

    /// Processes a keyboard event and updated the internal state.
    pub fn process_event(&mut self, key_state: ElementState, code: KeyCode) {
        match key_state {
            ElementState::Pressed => {
                self.state.insert(code, key_state);
            }
            ElementState::Released => {
                self.state.remove(&code);
            }
        }
    }
}
