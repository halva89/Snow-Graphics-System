use crate::core::scene::Scene;
use crate::render::Renderer;
use crate::platform::CustomWindow;
use crate::scene_parser::{SceneSettings, SceneObject};
use crate::animation::Animator;
use crate::math::Vec3;
use winit::keyboard::KeyCode;

pub struct Engine {
    scene: Scene,
    renderer: Renderer,
    window: CustomWindow,
    animator: Animator,
    is_running: bool,
    camera_pos: Vec3,
    camera_target: Vec3,
}

impl Engine {
    pub fn new(settings: SceneSettings) -> Self {
        let window = CustomWindow::new(&settings);
        let renderer = Renderer::new(&window);
        let scene = Scene::new();
        let animator = Animator::new();

        Self {
            scene,
            renderer,
            window,
            animator,
            is_running: true,
            camera_pos: Vec3::new(0.0, 0.0, 5.0),
            camera_target: Vec3::zero(),
        }
    }

    pub fn add_objects(&mut self, objects: Vec<SceneObject>) {
        for obj in objects {
            self.scene.add_object(obj);
        }
    }

    pub fn add_animation(&mut self, name: &str, keyframes: Vec<crate::animation::Keyframe>) {
        self.animator.add_animation(name, keyframes);
    }

    pub fn run(&mut self) {
        println!("Engine started");
        println!("Objects in scene: {}", self.scene.len());

        // Бесконечный цикл — закрываем через Ctrl+C
        loop {
            self.window.poll_events();
            
            // Управление камерой
            let speed = 3.0 * self.window.time.delta();
            let forward = (self.camera_target - self.camera_pos).normalize();
            let right = forward.cross(&Vec3::new(0.0, 0.0, 1.0)).normalize();
            
            // Используем стрелки для управления
            if self.window.input.is_key_pressed(KeyCode::KeyW) {
                self.camera_pos = self.camera_pos + forward * speed;
                self.camera_target = self.camera_target + forward * speed;
            }
            if self.window.input.is_key_pressed(KeyCode::KeyS) {
                self.camera_pos = self.camera_pos - forward * speed;
                self.camera_target = self.camera_target - forward * speed;
            }
            if self.window.input.is_key_pressed(KeyCode::KeyA) {
                self.camera_pos = self.camera_pos - right * speed;
                self.camera_target = self.camera_target - right * speed;
            }
            if self.window.input.is_key_pressed(KeyCode::KeyD) {
                self.camera_pos = self.camera_pos + right * speed;
                self.camera_target = self.camera_target + right * speed;
            }
            if self.window.input.is_key_pressed(KeyCode::KeyQ) {
                self.camera_pos.z += speed;
                self.camera_target.z += speed;
            }
            if self.window.input.is_key_pressed(KeyCode::KeyE) {
                self.camera_pos.z -= speed;
                self.camera_target.z -= speed;
            }
            
            self.renderer.set_camera(self.camera_pos, self.camera_target);
            
            // Обновляем анимации
            for object in self.scene.get_objects_mut() {
                if object.is_animated {
                    self.animator.apply(&mut object.transform, "main");
                }
            }
            
            self.renderer.render(&self.scene);
        }
    }

    pub fn shutdown(&mut self) {
        self.is_running = false;
        self.renderer.cleanup();
    }
}