use crate::core::scene::Scene;
use crate::render::Renderer;
use crate::platform::CustomWindow;
use crate::scene_parser::{SceneSettings, SceneObject, RenderMode};
use crate::animation::Animator;
use crate::math::Vec3;
<<<<<<< Updated upstream
use winit::keyboard::KeyCode;
use winit::application::ApplicationHandler;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::event::WindowEvent;
=======
use crate::physics::PhysicsWorld;
use rapier3d::na::vector;
use glfw::{Key, MouseButton};
use rayon::prelude::*;
>>>>>>> Stashed changes

pub struct Engine {
    settings: SceneSettings,
    scene: Scene,
    window: Option<CustomWindow>,
    renderer: Option<Renderer>,
    animator: Animator,
<<<<<<< Updated upstream
=======
    physics: Option<PhysicsWorld>,
>>>>>>> Stashed changes
    camera_pos: Vec3,
    camera_target: Vec3,
    yaw: f32,
    pitch: f32,
    camera_active: bool,
    dragging: bool,
    drag_ox: f32,
    drag_oy: f32,
    drag_wx: i32,
    drag_wy: i32,
    prev_mouse_down: bool,
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
<<<<<<< Updated upstream
=======
            physics: Some(PhysicsWorld::new()),
>>>>>>> Stashed changes
            camera_pos: Vec3::new(0.0, 0.0, 5.0),
            camera_target: Vec3::new(0.0, 0.0, 0.0),
            yaw: -90.0_f32.to_radians(),
            pitch: 0.0,
            camera_active,
            dragging: false,
            drag_ox: 0.0,
            drag_oy: 0.0,
            drag_wx: 0,
            drag_wy: 0,
            prev_mouse_down: false,
        }
    }

    pub fn add_objects(&mut self, objects: Vec<SceneObject>) {
<<<<<<< Updated upstream
        for obj in objects {
            self.scene.add_object(obj);
        }
    }

    pub fn add_animation(&mut self, name: &str, keyframes: Vec<crate::animation::Keyframe>) {
        self.animator.add_animation(name, keyframes);
=======
        for obj in objects.into_iter() {
            if let SceneObject::Animated(anim) = &obj {
                self.animator.add_animation(&anim.name, anim.keyframes.clone());
            }
            self.scene.add_object(obj);
        }
        // Physics bodies: один раз при создании сцены
        {
            let physics = self.physics.as_mut().unwrap();
            for object in self.scene.get_objects_mut() {
                if let Some(props) = object.physics.take() {
                    let (x, y, z) = object.transform.position;
                    let mesh = &object.mesh;
                    let half = if mesh.vertices.is_empty() {
                        (0.5, 0.5, 0.5)
                    } else {
                        let mut min = [f32::MAX; 3];
                        let mut max = [f32::MIN; 3];
                        for ch in mesh.vertices.chunks(3) {
                            for d in 0..3 {
                                if ch[d] < min[d] { min[d] = ch[d]; }
                                if ch[d] > max[d] { max[d] = ch[d]; }
                            }
                        }
                        (
                            (max[0] - min[0]) * 0.5,
                            (max[1] - min[1]) * 0.5,
                            (max[2] - min[2]) * 0.5,
                        )
                    };
                    let handle = physics.add_body(
                        vector![x, y, z],
                        vector![half.0, half.1, half.2],
                        props.mass,
                        props.restitution,
                        props.density,
                    );
                    object.physics_handle = Some(handle);
                }
            }
        }
>>>>>>> Stashed changes
    }

    pub fn run(&mut self) {
        println!("[Engine] Starting GLFW");
        let mut window = CustomWindow::new(&self.settings);
        let mut renderer = Renderer::new(&window, self.settings.render_mode == RenderMode::Heavy);
        println!("[Engine] Window + Vulkan ready");

        while !window.should_close() {
            window.poll_events();
            if window.input.is_key_pressed(Key::Escape) {
                window.set_should_close(true);
            }
            self.tick(&mut window, &mut renderer);
        }

        renderer.cleanup();
    }

    fn tick(&mut self, window: &mut CustomWindow, renderer: &mut Renderer) {
        let speed = 5.0 * window.time.delta();

// Панель: 40px, зазор 10px справа/сверху/снизу, кнопки 28x28 (красная ниже, жёлтая выше),
        // зазор 5px, отступ 5px от низа панели; скругление 13px
        let mx = window.input.mouse_x as i32;
        let my = window.input.mouse_y as i32;
        let scale_x = window.fb_width as f32 / window.width.max(1) as f32;
        let scale_y = window.fb_height as f32 / window.height.max(1) as f32;
        let mx_fb = (mx as f32 * scale_x) as i32;
        let my_fb = (my as f32 * scale_y) as i32;
        let fb_w = window.fb_width as i32;
        let fb_h = window.fb_height as i32;
        // Кнопки: панель от fb_w-50 до fb_w-10, кнопки с отступом 6px слева, ширина 28px
        let btn_left_px = fb_w - 10 - 40 + 6; // fb_w - 44
        let btn_right_px = btn_left_px + 28;  // fb_w - 16
        // Кнопки внизу: красная 15..43px от низа, жёлтая 48..76px от низа
        let red_bot = fb_h - 43;
        let red_top = fb_h - 15;
        let yel_bot = fb_h - 76;
        let yel_top = fb_h - 48;

        // Hover: 0 = нет, 1 = жёлтая (свернуть), 2 = красная (закрыть)
        let hover: u8 = if mx_fb >= btn_left_px && mx_fb <= btn_right_px {
            if my_fb >= red_bot && my_fb <= red_top {
                2
            } else if my_fb >= yel_bot && my_fb <= yel_top {
                1
            } else {
                0
            }
        } else {
            0
        };

        let mouse_down = window.input.is_mouse_down(MouseButton::Button1);
        let just_pressed = mouse_down && !self.prev_mouse_down;
        self.prev_mouse_down = mouse_down;

        if just_pressed {
            if hover == 2 {
                // Красная (нижняя) — закрыть
                window.set_should_close(true);
            } else if hover == 1 {
                // Жёлтая (выше) — свернуть
                window.iconify();
                return;
            } else if mx_fb >= fb_w - 50 && mx_fb <= fb_w - 10 && my_fb < fb_h - 76 {
                // Перетаскивание окна за правую панель (выше кнопок, во всю длину)
                self.dragging = true;
                self.drag_ox = window.input.mouse_x;
                self.drag_oy = window.input.mouse_y;
                let (wx, wy) = window.get_pos();
                self.drag_wx = wx;
                self.drag_wy = wy;
                #[cfg(windows)]
                window.native_drag();
            }
        }

        // На macOS/Linux drag идёт через ручной set_pos
        #[cfg(not(windows))]
        {
            if self.dragging && mouse_down {
                let dx = (window.input.mouse_x - self.drag_ox) as i32;
                let dy = (window.input.mouse_y - self.drag_oy) as i32;
                window.set_pos(self.drag_wx + dx, self.drag_wy + dy);
            }
            if !mouse_down {
                self.dragging = false;
            }
        }

        if self.camera_active {
<<<<<<< Updated upstream
            // Mouse look (only with right mouse button held)
            if window.input.mouse_buttons.contains(&winit::event::MouseButton::Right) {
=======
            if window.input.is_mouse_down(MouseButton::Button2) {
>>>>>>> Stashed changes
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

<<<<<<< Updated upstream
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
=======
            if window.input.is_key_pressed(Key::W) { self.camera_pos = self.camera_pos + forward * speed; }
            if window.input.is_key_pressed(Key::S) { self.camera_pos = self.camera_pos - forward * speed; }
            if window.input.is_key_pressed(Key::A) { self.camera_pos = self.camera_pos - right * speed; }
            if window.input.is_key_pressed(Key::D) { self.camera_pos = self.camera_pos + right * speed; }
            if window.input.is_key_pressed(Key::Q) { self.camera_pos.y += speed; }
            if window.input.is_key_pressed(Key::E) { self.camera_pos.y -= speed; }

>>>>>>> Stashed changes
            self.camera_target = self.camera_pos + front;

            renderer.set_camera(self.camera_pos, self.camera_target);
        }

<<<<<<< Updated upstream
        for object in self.scene.get_objects_mut() {
            if object.is_animated {
                self.animator.apply(&mut object.transform, "main");
            }
=======
        renderer.set_wireframe(self.settings.wireframe);
        let dt = window.time.delta();

        // 1. Физика: шаг и синк позиций в transform
        if let Some(physics) = self.physics.as_mut() {
            physics.step();
            for object in self.scene.get_objects_mut() {
                if let Some(handle) = object.physics_handle {
                    let pos = physics.get_body_pos(handle);
                    object.transform.set_position(pos.x, pos.y, pos.z);
                }
            }
        }

        // 2. Анимации: параллельно по объектам
        {
            let animator = &self.animator;
            let objects = self.scene.get_objects_mut();
            objects.par_iter_mut().for_each(|object| {
                if !object.is_animated || object.anim_name.is_empty() {
                    return;
                }
                object.anim_time = animator.advance_time(object.anim_time, dt, &object.anim_name);
                if let Some((pos, col)) = animator.sample(&object.anim_name, object.anim_time) {
                    object.transform.set_position(pos.x, pos.y, pos.z);
                    object.color = col;
                }
            });
>>>>>>> Stashed changes
        }

        renderer.render(&self.scene, window.fb_width, window.fb_height, hover);
        window.input.reset_delta();
    }
<<<<<<< Updated upstream
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
=======
>>>>>>> Stashed changes
}