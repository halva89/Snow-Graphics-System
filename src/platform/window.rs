use winit::{
    event::WindowEvent,
    event_loop::{EventLoop, ControlFlow},
    window::WindowBuilder,
    dpi::LogicalSize,
};
use crate::scene_parser::SceneSettings;
use crate::platform::input::Input;
use crate::platform::time::Time;

pub struct Window {
    pub event_loop: EventLoop<()>,
    pub raw: winit::window::Window,
    pub input: Input,
    pub time: Time,
    pub should_close: bool,
    pub width: u32,
    pub height: u32,
}

impl Window {
    pub fn new(settings: &SceneSettings) -> Self {
        let event_loop = EventLoop::new().unwrap();
        let raw = WindowBuilder::new()
            .with_title(&settings.title)
            .with_inner_size(LogicalSize::new(settings.width, settings.height))
            .build(&event_loop)
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

        self.event_loop.run_once(|event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                winit::event::Event::WindowEvent { event, .. } => {
                    self.input.handle_event(&event);
                    if let WindowEvent::CloseRequested = event {
                        self.should_close = true;
                        *control_flow = ControlFlow::Exit;
                    }
                    if let WindowEvent::Resized(size) = event {
                        self.width = size.width;
                        self.height = size.height;
                    }
                }
                _ => {}
            }
        }).unwrap_or_default();
    }

    pub fn swap_buffers(&mut self) {}
    pub fn should_close(&self) -> bool { self.should_close }
}