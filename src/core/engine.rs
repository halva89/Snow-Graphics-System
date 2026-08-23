use crate::core::scene::Scene;
use crate::render::Renderer;
use crate::platform::Window;
use crate::scene_parser::{SceneSettings, SceneObject};
use crate::animation::Animator;

pub struct Engine {
    scene: Scene,
    renderer: Renderer,
    window: Window,
    animator: Animator,
    is_running: bool,
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

        // Бесконечный цикл - закрывается только по Ctrl+C
        loop {
            self.window.poll_events();
            
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