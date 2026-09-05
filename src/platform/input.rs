use std::collections::HashSet;
use glfw::Action;
use glfw::Key;
use glfw::MouseButton;

pub struct Input {
    pub keys: HashSet<Key>,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub mouse_delta_x: f32,
    pub mouse_delta_y: f32,
    pub mouse_buttons: HashSet<MouseButton>,
}

impl Input {
    pub fn new() -> Self {
        Self {
            keys: HashSet::new(),
            mouse_x: 0.0,
            mouse_y: 0.0,
            mouse_delta_x: 0.0,
            mouse_delta_y: 0.0,
            mouse_buttons: HashSet::new(),
        }
    }

    pub fn key_callback(&mut self, key: Key, action: Action) {
        match action {
            Action::Press => { self.keys.insert(key); }
            Action::Release => { self.keys.remove(&key); }
            _ => {}
        }
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.keys.contains(&key)
    }

    pub fn mouse_button_callback(&mut self, btn: MouseButton, action: Action) {
        match action {
            Action::Press => { self.mouse_buttons.insert(btn); }
            Action::Release => { self.mouse_buttons.remove(&btn); }
            _ => {}
        }
    }

    pub fn is_mouse_down(&self, btn: MouseButton) -> bool {
        self.mouse_buttons.contains(&btn)
    }

    pub fn cursor_pos_callback(&mut self, x: f64, y: f64) {
        self.mouse_delta_x = x as f32 - self.mouse_x;
        self.mouse_delta_y = y as f32 - self.mouse_y;
        self.mouse_x = x as f32;
        self.mouse_y = y as f32;
    }

    pub fn reset_delta(&mut self) {
        self.mouse_delta_x = 0.0;
        self.mouse_delta_y = 0.0;
    }
}