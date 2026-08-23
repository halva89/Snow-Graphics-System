use winit::event::{WindowEvent, ElementState, KeyEvent, MouseButton};
use winit::keyboard::{KeyCode, PhysicalKey};
use std::collections::HashSet;

pub struct Input {
    pub keys: HashSet<KeyCode>,
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

    pub fn handle_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    physical_key: PhysicalKey::Code(keycode),
                    state,
                    ..
                },
                ..
            } => {
                match state {
                    ElementState::Pressed => {
                        self.keys.insert(*keycode);
                    }
                    ElementState::Released => {
                        self.keys.remove(keycode);
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                let old_x = self.mouse_x;
                let old_y = self.mouse_y;
                self.mouse_x = position.x as f32;
                self.mouse_y = position.y as f32;
                self.mouse_delta_x = self.mouse_x - old_x;
                self.mouse_delta_y = self.mouse_y - old_y;
            }
            WindowEvent::MouseInput { state, button, .. } => {
                match state {
                    ElementState::Pressed => {
                        self.mouse_buttons.insert(button.clone());
                    }
                    ElementState::Released => {
                        self.mouse_buttons.remove(button);
                    }
                }
            }
            _ => {}
        }
    }

    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.keys.contains(&key)
    }

    pub fn reset_delta(&mut self) {
        self.mouse_delta_x = 0.0;
        self.mouse_delta_y = 0.0;
    }
}