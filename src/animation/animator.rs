use crate::core::transform::Transform;
use crate::animation::Keyframe;
use crate::animation::interpolator::interpolate_keyframes;
use std::collections::HashMap;

pub struct Animator {
    animations: HashMap<String, Vec<Keyframe>>,
    current_time: f32,
    pub is_playing: bool,
    pub speed: f32,
    pub loop_enabled: bool,
}

impl Animator {
    pub fn new() -> Self {
        Self {
            animations: HashMap::new(),
            current_time: 0.0,
            is_playing: true,
            speed: 1.0,
            loop_enabled: true,
        }
    }

    pub fn add_animation(&mut self, name: &str, keyframes: Vec<Keyframe>) {
        self.animations.insert(name.to_string(), keyframes);
    }

    pub fn apply(&mut self, transform: &mut Transform, anim_name: &str) {
        if !self.is_playing {
            return;
        }

        let frames = match self.animations.get(anim_name) {
            Some(f) => f,
            None => return,
        };

        if frames.is_empty() {
            return;
        }

        // Используем фиксированный delta time (будет заменен на реальный)
        self.current_time += 0.016 * self.speed;

        let duration = frames.last().unwrap().time;
        if self.current_time > duration {
            if self.loop_enabled {
                self.current_time = 0.0;
            } else {
                return;
            }
        }

        for i in 0..frames.len() - 1 {
            let current = &frames[i];
            let next = &frames[i + 1];
            if self.current_time >= current.time && self.current_time < next.time {
                let t = (self.current_time - current.time) / (next.time - current.time);
                let (pos, _) = interpolate_keyframes(current, next, t);
                transform.set_position(pos.x, pos.y, pos.z);
                return;
            }
        }
    }

    pub fn reset(&mut self) {
        self.current_time = 0.0;
    }
}

impl Default for Animator {
    fn default() -> Self {
        Self::new()
    }
}