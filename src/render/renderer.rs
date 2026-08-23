use crate::render::vulkan::VulkanContext;
use crate::core::scene::Scene;
use crate::platform::Window;

pub struct Renderer {
    vulkan: VulkanContext,
    pub width: u32,
    pub height: u32,
}

impl Renderer {
    pub fn new(window: &Window) -> Self {
        let vulkan = VulkanContext::new(window);
        Self {
            vulkan,
            width: window.width,
            height: window.height,
        }
    }

    pub fn render(&mut self, scene: &Scene) {
        self.vulkan.render_scene(scene);
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    pub fn cleanup(&mut self) {
        self.vulkan.cleanup();
        println!("Renderer cleaned up");
    }
}