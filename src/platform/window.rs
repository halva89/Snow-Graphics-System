use glfw::{Glfw, PWindow, GlfwReceiver, WindowEvent, ClientApiHint};
use raw_window_handle::{HasWindowHandle, HasDisplayHandle, RawWindowHandle, RawDisplayHandle};
use crate::scene_parser::SceneSettings;
use crate::platform::input::Input;
use crate::platform::time::Time;

pub struct CustomWindow {
    pub glfw: Glfw,
    pub window: PWindow,
    pub events: GlfwReceiver<(f64, WindowEvent)>,
    pub input: Input,
    pub time: Time,
    pub should_close: bool,
    pub width: u32,
    pub height: u32,
    pub fb_width: u32,
    pub fb_height: u32,
}

impl CustomWindow {
    pub fn new(settings: &SceneSettings) -> Self {
        let mut glfw = glfw::init(glfw::log_errors).expect("GLFW init failed");
        glfw.window_hint(glfw::WindowHint::ClientApi(ClientApiHint::NoApi));
        glfw.window_hint(glfw::WindowHint::Resizable(true));
        // Хардкод: окно всегда без системных украшений (frameless), настройка игнорируется
        glfw.window_hint(glfw::WindowHint::Decorated(false));

        let (mut window, events) = glfw
            .create_window(settings.width, settings.height, &settings.title, glfw::WindowMode::Windowed)
            .expect("GLFW window creation failed");

        window.set_all_polling(true);

        // Закруглённые углы окна (только Windows)
        #[cfg(windows)]
        {
            let h = window.window_handle().unwrap().as_raw();
            if let raw_window_handle::RawWindowHandle::Win32(win) = h {
                let hwnd: isize = win.hwnd.get();
                unsafe {
                    let rounding = 2u32;
                    let hr = windows_sys::Win32::Graphics::Dwm::DwmSetWindowAttribute(
                        hwnd,
                        33,
                        &rounding as *const _ as *const std::ffi::c_void,
                        std::mem::size_of::<u32>() as u32,
                    );
                    if hr != 0 {
                        let (fw, fh) = window.get_framebuffer_size();
                        let rgn = windows_sys::Win32::Graphics::Gdi::CreateRoundRectRgn(0, 0, fw + 1, fh + 1, 13, 13);
                        if rgn != 0 {
                            windows_sys::Win32::Graphics::Gdi::SetWindowRgn(hwnd, rgn, 1);
                        }
                    }
                }
            }
        }

        let (w, h) = window.get_size();
        let (fw, fh) = window.get_framebuffer_size();
        Self {
            glfw,
            window,
            events,
            input: Input::new(),
            time: Time::new(),
            should_close: false,
            width: w as u32,
            height: h as u32,
            fb_width: fw as u32,
            fb_height: fh as u32,
        }
    }

    pub fn poll_events(&mut self) {
        self.time.update();
        self.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                WindowEvent::Close => self.should_close = true,
                WindowEvent::Key(key, _, action, _) => self.input.key_callback(key, action),
                WindowEvent::MouseButton(btn, action, _) => self.input.mouse_button_callback(btn, action),
                WindowEvent::CursorPos(x, y) => self.input.cursor_pos_callback(x, y),
                WindowEvent::Size(w, h) => {
                    self.width = w as u32;
                    self.height = h as u32;
                    let (fw, fh) = self.window.get_framebuffer_size();
                    self.fb_width = fw as u32;
                    self.fb_height = fh as u32;
                }
                _ => {}
            }
        }
    }

    pub fn should_close(&self) -> bool { self.should_close || self.window.should_close() }

    pub fn set_should_close(&mut self, val: bool) {
        self.should_close = val;
        self.window.set_should_close(val);
    }

    pub fn get_pos(&self) -> (i32, i32) { self.window.get_pos() }

    pub fn set_pos(&mut self, x: i32, y: i32) { self.window.set_pos(x, y); }

    pub fn iconify(&mut self) { self.window.iconify(); }

    pub fn maximize(&mut self) { self.window.maximize(); }

    pub fn restore(&mut self) { self.window.restore(); }

    pub fn is_maximized(&self) -> bool { self.window.is_maximized() }

    pub fn native_drag(&self) {
        #[cfg(windows)]
        {
            let h = self.window.window_handle().unwrap().as_raw();
            if let raw_window_handle::RawWindowHandle::Win32(win) = h {
                let hwnd: isize = win.hwnd.get();
                unsafe {
                    // Release capture so the drag doesn't conflict with GLFW's mouse tracking
                    windows_sys::Win32::UI::Input::KeyboardAndMouse::ReleaseCapture();
                    windows_sys::Win32::UI::WindowsAndMessaging::SendMessageW(
                        hwnd,
                        windows_sys::Win32::UI::WindowsAndMessaging::WM_NCLBUTTONDOWN,
                        windows_sys::Win32::UI::WindowsAndMessaging::HTCAPTION as usize,
                        0,
                    );
                }
            }
        }
        #[cfg(not(windows))]
        {
            // On macOS/Linux, no fallback
        }
    }

    pub fn dummy(w: u32, h: u32) -> Self {
        let mut glfw = glfw::init(glfw::log_errors).expect("GLFW init failed");
        glfw.window_hint(glfw::WindowHint::ClientApi(ClientApiHint::NoApi));
        let (window, events) = glfw.create_window(w.max(1), h.max(1), "dummy", glfw::WindowMode::Windowed)
            .expect("GLFW dummy window failed");
        let (fw, fh) = window.get_framebuffer_size();
        Self {
            glfw, window, events,
            input: Input::new(), time: Time::new(),
            should_close: false, width: w, height: h,
            fb_width: fw as u32, fb_height: fh as u32,
        }
    }

    pub fn get_raw_window_handle(&self) -> RawWindowHandle {
        self.window.window_handle().unwrap().as_raw()
    }

    pub fn get_raw_display_handle(&self) -> RawDisplayHandle {
        self.window.display_handle().unwrap().as_raw()
    }
}