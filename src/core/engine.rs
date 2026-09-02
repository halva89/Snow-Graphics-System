use crate::core::scene::Scene;
use crate::render::Renderer;
use crate::platform::CustomWindow;
use crate::scene_parser::{SceneSettings, SceneObject, RenderMode};
use crate::animation::Animator;
use crate::math::Vec3;
use crate::physics::PhysicsWorld;
use crate::physics::PhysicsProps;
use rapier3d::na::vector;
use rapier3d::prelude::RigidBodyHandle;
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
    physics: PhysicsWorld,
    body_handles: Vec<RigidBodyHandle>,
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
            physics: PhysicsWorld::new(),
            body_handles: Vec::new(),
            camera_pos: Vec3::new(0.0, 0.0, 5.0),
            camera_target: Vec3::new(0.0, 0.0, 0.0),
            yaw: -90.0_f32.to_radians(),
            pitch: 0.0,
            camera_active,
        }
    }

    pub fn add_objects(&mut self, objects: Vec<SceneObject>) {
        for (i, obj) in objects.into_iter().enumerate() {
            if let SceneObject::Animated(anim) = &obj {
                self.animator.add_animation(&format!("anim_{}", i), anim.keyframes.clone());
            }
            self.scene.add_object(obj);
        }

        for obj in self.scene.get_objects() {
            if obj.physics.is_some() {
                let pos = obj.transform.position;
                let half = 0.5;
                let h = self.physics.add_body(
                    vector![pos.0, pos.1, pos.2],
                    vector![half, half, half],
                    obj.physics.as_ref().unwrap().mass,
                    obj.physics.as_ref().unwrap().restitution,
                );
                self.body_handles.push(h);
            }
        }
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
            if window.input.mouse_buttons.contains(&winit::event::MouseButton::Right) {
                let dx = window.input.mouse_delta_x;
                let dy = window.input.mouse_delta_y;
                self.yaw += dx * 0.002;
                self.pitch -= dy * 0.002;
                self.pitch = self.pitch.clamp(-1.5, 1.5);
            }

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
            self.camera_target = self.camera_pos + front;
            renderer.set_camera(self.camera_pos, self.camera_target);
        }

        let dt = window.time.delta();
        for object in self.scene.get_objects_mut() {
            if object.is_animated {
                let an = object.anim_name.clone();
                self.animator.apply(&mut object.transform, &an, dt);
                if let Some(c) = self.animator.get_current_color(&an) {
                    object.color = c;
                }
            }
        }

        // Step physics and sync positions
        self.physics.step();
        let mut body_idx = 0;
        for obj in self.scene.get_objects_mut() {
            if body_idx >= self.body_handles.len() { break; }
            if obj.physics.is_some() {
                let h = self.body_handles[body_idx];
                let p = self.physics.get_body_pos(h);
                println!("[Physics] Body {} pos: y={}", body_idx, p.y);
                obj.transform.set_position(p.x, p.y, p.z);
                body_idx += 1;
            }
        }

        renderer.render(&self.scene, window.width, window.height);
        window.input.reset_delta();
    }
}

impl ApplicationHandler for Engine {
    fn resumed(&mut self, elwt: &ActiveEventLoop) {
        if self.window.is_some() { return; }

        let mut window = CustomWindow::new(&self.settings);
        window.init_window(elwt);
        let renderer = Renderer::new(&window, self.settings.render_mode == RenderMode::Heavy);

        println!("[Engine] Window + Vulkan ready");
        self.window = Some(window);
        self.renderer = Some(renderer);
    }

    fn window_event(&mut self, elwt: &ActiveEventLoop, _: winit::window::WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => elwt.exit(),
            _ => {
                if let Some(window) = self.window.as_mut() {
                    window.handle_event(&event);
                }
            }
        }
    }

    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        self.render_frame();
    }
}