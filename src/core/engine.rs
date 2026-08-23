use crate::core::scene::Scene;
use crate::render::Renderer;
use crate::platform::CustomWindow;
use crate::scene_parser::{SceneSettings, SceneObject};
use crate::animation::Animator;
use crate::math::Vec3;

pub struct Engine {
    scene: Scene,
    renderer: Renderer,
    window: CustomWindow,
    animator: Animator,
    is_running: bool,
    camera_pos: Vec3,
    camera_target: Vec3,
    frame_count: u32,
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
            frame_count: 0,
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

        while !self.window.should_close() {
            self.window.poll_events();
            
            // Камера статична
            self.renderer.set_camera(self.camera_pos, self.camera_target);
            
            for object in self.scene.get_objects_mut() {
                if object.is_animated {
                    self.animator.apply(&mut object.transform, "main");
                }
            }
            
            self.renderer.render(&self.scene);
            self.frame_count += 1;
        }

        self.shutdown();
        println!("Engine stopped");
    }

    pub fn shutdown(&mut self) {
        self.is_running = false;
        self.renderer.cleanup();
    }
}