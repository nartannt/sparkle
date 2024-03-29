use std::collections::HashMap;
use winit::event::ElementState;
use winit::keyboard::KeyCode;

// this code was stolen from https://github.com/rust-windowing/glutin/issues/708 because i couldn't
// be asked to write it myself
/// Keeps track of which keys have been pressed.
pub struct KeyboardState {
    state: HashMap<KeyCode, ElementState>,
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
