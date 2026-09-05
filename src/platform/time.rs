use std::time::Instant;

pub struct Time {
    pub start_time: Instant,
    pub last_frame_time: Instant,
    pub delta_time: f32,
    pub total_time: f32,
    pub frame_count: u32,
    pub fps: f32,
    fps_counter: u32,
    fps_timer: f32,
}

impl Time {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            start_time: now,
            last_frame_time: now,
            delta_time: 0.0,
            total_time: 0.0,
            frame_count: 0,
            fps: 0.0,
            fps_counter: 0,
            fps_timer: 0.0,
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        self.delta_time = now.duration_since(self.last_frame_time).as_secs_f32();
        self.last_frame_time = now;
        self.total_time += self.delta_time;
        self.frame_count += 1;
        self.fps_counter += 1;
        self.fps_timer += self.delta_time;
        if self.fps_timer >= 1.0 {
            self.fps = self.fps_counter as f32 / self.fps_timer;
            self.fps_counter = 0;
            self.fps_timer = 0.0;
        }
    }

    pub fn delta(&self) -> f32 { self.delta_time }
    pub fn total(&self) -> f32 { self.total_time }
    pub fn fps(&self) -> f32 { self.fps }
}