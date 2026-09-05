use crate::render::vulkan::VulkanContext;
use crate::core::scene::Scene;
use crate::platform::CustomWindow;
use crate::math::Vec3;

pub struct Renderer {
    vulkan: VulkanContext,
    pub width: u32,
    pub height: u32,
}

impl Renderer {
    pub fn new(window: &CustomWindow, prefer_discrete: bool) -> Self {
        let vulkan = VulkanContext::new(window, prefer_discrete);
        Self {
            vulkan,
            width: window.width,
            height: window.height,
        }
    }

    pub fn set_camera(&mut self, eye: Vec3, target: Vec3) {
        self.vulkan.set_camera(eye, target);
    }

    pub fn set_wireframe(&mut self, on: bool) {
        self.vulkan.set_wireframe(on);
    }

    pub fn render(&mut self, scene: &Scene, w: u32, h: u32, hover: u8) {
        self.vulkan.render_scene(scene, w, h, hover);
    }

    pub fn cleanup(&mut self) {
        self.vulkan.cleanup();
    }
}