use crate::core::scene::Scene;
use crate::render::Renderer;
use crate::platform::CustomWindow;
use crate::scene_parser::{self, SceneSettings, SceneObject, RenderMode};
use crate::animation::Animator;
use crate::math::Vec3;
use crate::physics::PhysicsWorld;
use crate::physics::PhysicsProps;
use rapier3d::na::vector;
<<<<<<< Updated upstream
use rapier3d::prelude::RigidBodyHandle;
use winit::keyboard::KeyCode;
use winit::application::ApplicationHandler;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::event::WindowEvent;
=======
use glfw::{Key, MouseButton};
use rayon::prelude::*;
use std::time::{SystemTime, Duration};
>>>>>>> Stashed changes

pub struct Engine {
    settings: SceneSettings,
    scene: Scene,
    window: Option<CustomWindow>,
    renderer: Option<Renderer>,
    animator: Animator,
<<<<<<< Updated upstream
    physics: PhysicsWorld,
    body_handles: Vec<RigidBodyHandle>,
=======
    physics: Option<PhysicsWorld>,
    scene_path: String,
    last_mtime: Option<SystemTime>,
    reload_check: Duration,
>>>>>>> Stashed changes
    camera_pos: Vec3,
    camera_target: Vec3,
    yaw: f32,
    pitch: f32,
    camera_active: bool,
}

impl Engine {
    pub fn run_from(path: String) {
        let (settings, objects) = scene_parser::load_scene(&path);
        let mut engine = Engine::new_with_path(settings, path);
        engine.add_objects(objects);
        engine.run();
    }

    pub fn new(settings: SceneSettings) -> Self {
        Self::new_with_path(settings, String::new())
    }

    pub fn new_with_path(settings: SceneSettings, scene_path: String) -> Self {
        let camera_active = settings.camera_enabled;
        let mut engine = Self {
            settings,
            scene: Scene::new(),
            window: None,
            renderer: None,
            animator: Animator::new(),
<<<<<<< Updated upstream
            physics: PhysicsWorld::new(),
            body_handles: Vec::new(),
=======
            physics: Some(PhysicsWorld::new()),
            scene_path,
            last_mtime: None,
            reload_check: Duration::from_secs(0),
>>>>>>> Stashed changes
            camera_pos: Vec3::new(0.0, 0.0, 5.0),
            camera_target: Vec3::new(0.0, 0.0, 0.0),
            yaw: -90.0_f32.to_radians(),
            pitch: 0.0,
            camera_active,
<<<<<<< Updated upstream
=======
            dragging: false,
            drag_ox: 0.0,
            drag_oy: 0.0,
            drag_wx: 0,
            drag_wy: 0,
            prev_mouse_down: false,
        };
        if !engine.scene_path.is_empty() {
            engine.reload_scene();
        }
        // Стартовая камера из настроек сцены: строим yaw/pitch так, чтобы камера
        // смотрела из camera_pos на camera_target (по умолчанию совпадает с
        // прежним поведением — спереди на центр).
        {
            let (px, py, pz) = engine.settings.camera_pos;
            let (tx, ty, tz) = engine.settings.camera_target;
            engine.camera_pos = Vec3::new(px, py, pz);
            engine.camera_target = Vec3::new(tx, ty, tz);
            let (dx, dy, dz) = (tx - px, ty - py, tz - pz);
            let horiz = (dx * dx + dz * dz).sqrt();
            engine.pitch = if horiz > 1e-6 {
                dy.atan2(horiz)
            } else if dy >= 0.0 {
                std::f32::consts::FRAC_PI_2
            } else {
                -std::f32::consts::FRAC_PI_2
            };
            engine.yaw = dz.atan2(dx);
        }
        engine
    }

    // Перезагружает сцену из .mff: объекты, анимации, физика, настройки окна.
    // Камера и её позиция сохраняются.
    fn reload_scene(&mut self) {
        if self.scene_path.is_empty() { return; }
        println!("[Engine] Reloading scene: {}", self.scene_path);
        match scene_parser::load_scene(&self.scene_path) {
            (settings, objects) => {
                self.settings = settings;
                self.camera_active = self.settings.camera_enabled;
                self.scene = Scene::new();
                self.animator = Animator::new();
                self.physics = Some(PhysicsWorld::new());
                self.add_objects(objects);
                // Применяем новые настройки к уже созданному окну/рендереру
                if let Some(renderer) = self.renderer.as_mut() {
                    renderer.set_background(self.settings.background);
                    renderer.set_wireframe(self.settings.wireframe);
                    renderer.set_thermal(self.settings.thermal);
                }
                println!("[Engine] Scene reloaded: {} objects", self.scene.len());
            }
        }
    }

    // Проверяет изменился ли файл сцены; если да — перезагружает.
    // Проверка не чаще раза в ~0.25 сек.
    fn check_reload(&mut self, dt: Duration) {
        if self.scene_path.is_empty() { return; }
        self.reload_check += dt;
        if self.reload_check < Duration::from_millis(250) { return; }
        self.reload_check = Duration::from_secs(0);

        let mtime = std::fs::metadata(&self.scene_path)
            .and_then(|m| m.modified()).ok();
        if let Some(mt) = mtime {
            if self.last_mtime != Some(mt) {
                // Файл мог быть ещё не дописан — ждём 150мс и пробуем снова
                std::thread::sleep(Duration::from_millis(150));
                self.last_mtime = std::fs::metadata(&self.scene_path)
                    .and_then(|m| m.modified()).ok();
                self.reload_scene();
            }
        } else if self.last_mtime.is_none() {
            // файл ещё не существует — первый раз запомним
            self.last_mtime = SystemTime::now().into();
>>>>>>> Stashed changes
        }
    }

    pub fn add_objects(&mut self, objects: Vec<SceneObject>) {
<<<<<<< Updated upstream
        for (i, obj) in objects.into_iter().enumerate() {
            if let SceneObject::Animated(anim) = &obj {
                self.animator.add_animation(&format!("anim_{}", i), anim.keyframes.clone());
=======
        for obj in objects.into_iter() {
            if let SceneObject::Animated(anim, _) = &obj {
                self.animator.add_animation(&anim.name, anim.keyframes.clone());
>>>>>>> Stashed changes
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

<<<<<<< Updated upstream
    pub fn run(mut self) {
        println!("[Engine] Starting event loop");
        let event_loop = EventLoop::new().unwrap();
        let _ = event_loop.run_app(&mut self);
=======
    pub fn run(&mut self) {
        println!("[Engine] Starting GLFW");
        let mut window = CustomWindow::new(&self.settings);
        let mut renderer = Renderer::new(&window, self.settings.render_mode == RenderMode::Heavy, self.settings.aa_type, self.settings.thermal);
        renderer.set_background(self.settings.background);
        println!("[Engine] Window + Vulkan ready");

        // Load scene texture if specified
        if let Some(ref tex_path) = self.settings.texture.clone() {
            let path = std::path::Path::new(tex_path);
            let full_path = if path.is_absolute() {
                tex_path.clone()
            } else {
                let base = std::env::current_dir().unwrap_or_default();
                base.join(tex_path).to_string_lossy().to_string()
            };
            renderer.load_texture(&full_path);
        }

        while !window.should_close() {
            window.poll_events();
            if window.input.is_key_pressed(Key::Escape) {
                window.set_should_close(true);
            }
            self.tick(&mut window, &mut renderer);
        }

        renderer.cleanup();
>>>>>>> Stashed changes
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
<<<<<<< Updated upstream
        for object in self.scene.get_objects_mut() {
            if object.is_animated {
                let an = object.anim_name.clone();
                self.animator.apply(&mut object.transform, &an, dt);
                if let Some(c) = self.animator.get_current_color(&an) {
                    object.color = c;
=======
        self.check_reload(Duration::from_secs_f32(dt));

        if let Some(physics) = self.physics.as_mut() {
            physics.step();
            for object in self.scene.get_objects_mut() {
                if let Some(handle) = object.physics_handle {
                    let pos = physics.get_body_pos(handle);
                    object.transform.set_position(pos.x, pos.y, pos.z);
>>>>>>> Stashed changes
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

<<<<<<< Updated upstream
        renderer.render(&self.scene, window.width, window.height);
=======
        renderer.render(&self.scene, window.fb_width, window.fb_height, hover, dt);
>>>>>>> Stashed changes
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