use crate::core::scene::Scene;
use crate::render::Renderer;
use crate::platform::CustomWindow;
use crate::scene_parser::{SceneSettings, SceneObject, RenderMode};
use crate::animation::Animator;
use crate::math::Vec3;
use winit::keyboard::KeyCode;
use winit::application::ApplicationHandler;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::event::WindowEvent;

pub struct Engine {
    settings: SceneSettings,
    scene: Scene,
    window: Option<CustomWindow>,
    renderer: Option<Renderer>,
    animator: Animator,
    camera_pos: Vec3,
    camera_target: Vec3,
    yaw: f32,
    pitch: f32,
    camera_active: bool,
}

impl Engine {
    pub fn new(settings: SceneSettings) -> Self {
        let camera_active = settings.camera_enabled;
        Self {
            settings,
            scene: Scene::new(),
            window: None,
            renderer: None,
            animator: Animator::new(),
            camera_pos: Vec3::new(0.0, 0.0, 5.0),
            camera_target: Vec3::new(0.0, 0.0, 0.0),
            yaw: -90.0_f32.to_radians(),
            pitch: 0.0,
            camera_active,
        }
    }

    pub fn add_objects(&mut self, objects: Vec<SceneObject>) {
        let positions = [
            (0.0, 0.0, 0.0),   // cube
            (1.2, 0.0, 0.0),   // sphere
            (-2.0, 0.0, 0.5),  // square
        ];
        for (i, obj) in objects.into_iter().enumerate() {
            self.scene.add_object(obj);
            if let Some(pos) = positions.get(i) {
                let last = self.scene.get_objects_mut().len() - 1;
                self.scene.get_objects_mut()[last].set_position(pos.0, pos.1, pos.2);
            }
        }
    }

    pub fn add_animation(&mut self, name: &str, keyframes: Vec<crate::animation::Keyframe>) {
        self.animator.add_animation(name, keyframes);
    }

    pub fn run(mut self) {
        println!("[Engine] Starting event loop");
        let event_loop = EventLoop::new().unwrap();
        let _ = event_loop.run_app(&mut self);
    }

    fn render_frame(&mut self) {
        let Some(window) = self.window.as_mut() else { return };
        let Some(renderer) = self.renderer.as_mut() else { return };

        window.time.update();
        let speed = 5.0 * window.time.delta();

        if self.camera_active {
            // Mouse look (only with right mouse button held)
            if window.input.mouse_buttons.contains(&winit::event::MouseButton::Right) {
                let dx = window.input.mouse_delta_x;
                let dy = window.input.mouse_delta_y;
                self.yaw += dx * 0.002;
                self.pitch -= dy * 0.002;
                self.pitch = self.pitch.clamp(-1.5, 1.5);
            }

            // Always update forward/target from yaw/pitch
            let front = Vec3::new(
                self.yaw.cos() * self.pitch.cos(),
                self.pitch.sin(),
                self.yaw.sin() * self.pitch.cos(),
            ).normalize();
            self.camera_target = self.camera_pos + front;

            let forward = front;
            let right = forward.cross(&Vec3::new(0.0, 1.0, 0.0)).normalize();

            if window.input.is_key_pressed(KeyCode::KeyW) {
                self.camera_pos = self.camera_pos + forward * speed;
            }
            if window.input.is_key_pressed(KeyCode::KeyS) {
                self.camera_pos = self.camera_pos - forward * speed;
            }
            if window.input.is_key_pressed(KeyCode::KeyA) {
                self.camera_pos = self.camera_pos - right * speed;
            }
            if window.input.is_key_pressed(KeyCode::KeyD) {
                self.camera_pos = self.camera_pos + right * speed;
            }
            if window.input.is_key_pressed(KeyCode::KeyQ) {
                self.camera_pos.y += speed;
            }
            if window.input.is_key_pressed(KeyCode::KeyE) {
                self.camera_pos.y -= speed;
            }

            // Recalc target after movement
            self.camera_target = self.camera_pos + front;

            renderer.set_camera(self.camera_pos, self.camera_target);
        }

        for object in self.scene.get_objects_mut() {
            if object.is_animated {
                self.animator.apply(&mut object.transform, "main");
            }
        }

        renderer.render(&self.scene, window.width, window.height);
        window.input.reset_delta();
    }
}

impl ApplicationHandler for Engine {
    fn resumed(&mut self, elwt: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let mut window = CustomWindow::new(&self.settings);
        window.init_window(elwt);

        let renderer = Renderer::new(&window, self.settings.render_mode == RenderMode::Heavy);

        println!("[Engine] Window + Vulkan ready");
        self.window = Some(window);
        self.renderer = Some(renderer);
    }

    fn window_event(
        &mut self,
        elwt: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                elwt.exit();
            }
            _ => {
                if let Some(window) = self.window.as_mut() {
                    window.handle_event(&event);
                }
            }
        }
    }

    fn about_to_wait(&mut self, _elwt: &ActiveEventLoop) {
        self.render_frame();
    }
}