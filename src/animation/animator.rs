use crate::animation::Keyframe;
use crate::animation::interpolator::interpolate_keyframes;
use crate::math::Vec3;
use crate::types::Color;
use std::collections::HashMap;

pub struct Animator {
    animations: HashMap<String, Vec<Keyframe>>,
    pub is_playing: bool,
    pub speed: f32,
    pub loop_enabled: bool,
}

impl Animator {
    pub fn new() -> Self {
        Self {
            animations: HashMap::new(),
            is_playing: true,
            speed: 1.0,
            loop_enabled: true,
        }
    }

    pub fn add_animation(&mut self, name: &str, keyframes: Vec<Keyframe>) {
        self.animations.insert(name.to_string(), keyframes);
    }

<<<<<<< Updated upstream
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
=======
    pub fn sample(&self, name: &str, current_time: f32) -> Option<(Vec3, Color)> {
        let frames = self.animations.get(name)?;
        if frames.is_empty() {
            return None;
        }
        let duration = frames.last().unwrap().time;
        if duration <= 0.0 {
            return Some((frames[0].position, frames[0].color));
        }
        let t = current_time.min(duration);

        if t <= frames[0].time {
            return Some((frames[0].position, frames[0].color));
        }

        for i in 0..frames.len() - 1 {
            let cur = &frames[i];
            let nxt = &frames[i + 1];
            if t >= cur.time && t < nxt.time {
                let frac = if nxt.time - cur.time > 0.0 {
                    (t - cur.time) / (nxt.time - cur.time)
                } else {
                    0.0
                };
                return Some(interpolate_keyframes(cur, nxt, frac));
            }
        }

        let last = frames.last().unwrap();
        Some((last.position, last.color))
    }

    pub fn advance_time(&self, current_time: f32, dt: f32, name: &str) -> f32 {
        if !self.is_playing {
            return current_time;
        }
        let frames = match self.animations.get(name) {
            Some(f) => f,
            None => return current_time,
        };
        if frames.is_empty() {
            return current_time;
        }
        let duration = frames.last().unwrap().time;
        if duration <= 0.0 {
            return current_time;
        }
        let mut t = current_time + dt * self.speed;
        if t > duration {
            if self.loop_enabled {
                t -= duration;
            } else {
                t = duration;
            }
        }
        t
    }

    pub fn get_duration(&self, name: &str) -> Option<f32> {
        let frames = self.animations.get(name)?;
        if frames.is_empty() { None } else { Some(frames.last().unwrap().time) }
>>>>>>> Stashed changes
    }

    pub fn reset(&mut self) {
    }
}

impl Default for Animator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::Vec3;
    use crate::types::Color;

    fn make_keyframes() -> Vec<Keyframe> {
        vec![
            Keyframe::new(0.0, Vec3::new(0.0, 0.0, 0.0), Color::new(1.0, 0.0, 0.0)),
            Keyframe::new(1.0, Vec3::new(1.0, 1.0, 0.0), Color::new(0.0, 1.0, 0.0)),
            Keyframe::new(2.0, Vec3::new(0.0, 0.0, 0.0), Color::new(0.0, 0.0, 1.0)),
        ]
    }

    #[test]
    fn test_sample_and_advance() {
        let mut anim = Animator::new();
        anim.add_animation("a", make_keyframes());
        anim.add_animation("b", make_keyframes());

        // Per-object times
        let mut t_a = 0.0f32;
        let mut t_b = 0.0f32;

        // Advance only "a" by 1.0s
        t_a = anim.advance_time(t_a, 1.0, "a");
        let (pos_a, _) = anim.sample("a", t_a).unwrap();
        assert!((pos_a.x - 1.0).abs() < 0.001, "a should be at x=1.0");

        // "b" still at 0
        let (pos_b, _) = anim.sample("b", t_b).unwrap();
        assert!((pos_b.x - 0.0).abs() < 0.001, "b should still be at origin");

        // Advance "b" too
        t_b = anim.advance_time(t_b, 1.0, "b");
        let (pos_b, _) = anim.sample("b", t_b).unwrap();
        assert!((pos_b.x - 1.0).abs() < 0.001, "b should have moved");
    }

    #[test]
    fn test_diff_durations() {
        let short = vec![
            Keyframe::new(0.0, Vec3::zero(), Color::white()),
            Keyframe::new(1.0, Vec3::new(1.0, 0.0, 0.0), Color::white()),
        ];
        let long = vec![
            Keyframe::new(0.0, Vec3::zero(), Color::white()),
            Keyframe::new(5.0, Vec3::new(0.0, 1.0, 0.0), Color::white()),
        ];

        let mut anim = Animator::new();
        anim.loop_enabled = true;
        anim.add_animation("short", short);
        anim.add_animation("long", long);

        let mut t_s = 0.0f32;
        let mut t_l = 0.0f32;

        t_s = anim.advance_time(t_s, 1.5, "short"); // loops: 1.5-1.0=0.5
        t_l = anim.advance_time(t_l, 1.5, "long");  // t=1.5

        let (pos_s, _) = anim.sample("short", t_s).unwrap();
        assert!((pos_s.x - 0.5).abs() < 0.001, "short looped to 0.5");

        let (pos_l, _) = anim.sample("long", t_l).unwrap();
        assert!((pos_l.y - 0.3).abs() < 0.001, "long should be at its own time");
    }

    #[test]
    fn test_loop() {
        let mut anim = Animator::new();
        anim.loop_enabled = true;
        anim.add_animation("a", make_keyframes());

        let mut t = 0.0f32;
        t = anim.advance_time(t, 3.0, "a"); // 3.0 - 2.0 = 1.0
        let (pos, _) = anim.sample("a", t).unwrap();
        assert!((pos.x - 1.0).abs() < 0.001);
        assert!((pos.y - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_color_interpolation() {
        let mut anim = Animator::new();
        anim.add_animation("a", make_keyframes());

        let mut t = 0.0f32;
        t = anim.advance_time(t, 0.5, "a");
        let (_, col) = anim.sample("a", t).unwrap();
        assert!((col.r - 0.5).abs() < 0.001);
        assert!((col.g - 0.5).abs() < 0.001);
        assert!((col.b - 0.0).abs() < 0.001);
    }
}