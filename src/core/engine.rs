use crate::core::scene::Scene;
use crate::render::Renderer;
use crate::platform::window::Window;
use crate::scene_parser::{SceneSettings, SceneObject};
use crate::animation::Animator;

pub struct Engine {
    scene: Scene,
    renderer: Renderer,
    window: Window,
    animator: Animator,
    is_running: bool,
    pub fps: u32,
}

impl Engine {
    pub fn new(settings: SceneSettings) -> Self {
        let window = Window::new(&settings);
        let renderer = Renderer::new(&window);
        let scene = Scene::new();
        let animator = Animator::new();

        Self {
            scene,
            renderer,
            window,
            animator,
            is_running: true,
            fps: 60,
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

        while self.is_running {
            self.window.poll_events();

            for obj in self.scene.get_objects_mut() {
                if obj.is_animated {
                    self.animator.apply(&mut obj.transform, "default");
                }
            }

            self.renderer.render(&self.scene);
            self.window.swap_buffers();
        }

        println!("Engine stopped");
    }

    pub fn shutdown(&mut self) {
        self.is_running = false;
    }
}