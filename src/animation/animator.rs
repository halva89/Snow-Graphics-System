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

    pub fn apply(&mut self, transform: &mut Transform, anim_name: &str, delta_time: f32) -> bool {
        if !self.is_playing {
            return false;
        }

        let frames = match self.animations.get(anim_name) {
            Some(f) => f,
            None => return false,
        };

        if frames.is_empty() {
            return false;
        }

        self.current_time += delta_time * self.speed;

        let duration = frames.last().unwrap().time;
        if duration <= 0.0 {
            return false;
        }

        if self.current_time > duration {
            if self.loop_enabled {
                self.current_time -= duration;
            } else {
                self.current_time = duration;
            }
        }

        if self.current_time < frames[0].time {
            transform.set_position(frames[0].position.x, frames[0].position.y, frames[0].position.z);
            return true;
        }

        for i in 0..frames.len() - 1 {
            let cur = &frames[i];
            let nxt = &frames[i + 1];
            if self.current_time >= cur.time && self.current_time < nxt.time {
                let t = if nxt.time - cur.time > 0.0 {
                    (self.current_time - cur.time) / (nxt.time - cur.time)
                } else {
                    0.0
                };
                let (pos, _) = interpolate_keyframes(cur, nxt, t);
                transform.set_position(pos.x, pos.y, pos.z);
                return true;
            }
        }

        let last = frames.last().unwrap();
        transform.set_position(last.position.x, last.position.y, last.position.z);
        true
    }

    pub fn get_current_color(&self, anim_name: &str) -> Option<crate::types::Color> {
        let frames = self.animations.get(anim_name)?;
        if frames.is_empty() {
            return None;
        }

        let ct = self.current_time;
        let duration = frames.last().unwrap().time;
        if duration <= 0.0 {
            return Some(frames[0].color);
        }

        if frames.len() == 1 {
            return Some(frames[0].color);
        }

        for i in 0..frames.len() - 1 {
            let cur = &frames[i];
            let nxt = &frames[i + 1];
            if ct >= cur.time && ct < nxt.time {
                let t = if nxt.time - cur.time > 0.0 {
                    (ct - cur.time) / (nxt.time - cur.time)
                } else {
                    0.0
                };
                let (_, col) = interpolate_keyframes(cur, nxt, t);
                return Some(col);
            }
        }
        Some(frames.last().unwrap().color)
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