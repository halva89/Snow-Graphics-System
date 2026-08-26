use winit::{
    window::Window,
    event_loop::EventLoop,
    dpi::LogicalSize,
};
use raw_window_handle::{HasWindowHandle, HasDisplayHandle, RawWindowHandle, RawDisplayHandle};
use crate::scene_parser::SceneSettings;
use crate::platform::input::Input;
use crate::platform::time::Time;

pub struct CustomWindow {
    pub event_loop: EventLoop<()>,
    pub raw: Window,
    pub input: Input,
    pub time: Time,
    pub should_close: bool,
    pub width: u32,
    pub height: u32,
}

impl CustomWindow {
    pub fn new(settings: &SceneSettings) -> Self {
        let event_loop = EventLoop::new().unwrap();
        let raw = event_loop
            .create_window(
                Window::default_attributes()
                    .with_title(&settings.title)
                    .with_inner_size(LogicalSize::new(settings.width, settings.height)),
            )
            .unwrap();

        Self {
            event_loop,
            raw,
            input: Input::new(),
            time: Time::new(),
            should_close: false,
            width: settings.width,
            height: settings.height,
        }
    }

    pub fn poll_events(&mut self) {
        self.time.update();
        // Без событий — просто обновляем время
    }

    pub fn should_close(&self) -> bool {
        self.should_close
    }

    pub fn set_should_close(&mut self, value: bool) {
        self.should_close = value;
    }

    pub fn get_raw_window_handle(&self) -> RawWindowHandle {
        self.raw.window_handle().unwrap().as_raw()
    }

    pub fn get_raw_display_handle(&self) -> RawDisplayHandle {
        self.raw.display_handle().unwrap().as_raw()
    }
}