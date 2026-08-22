use crate::render::vulkan::VulkanContext;
use crate::core::scene::Scene;
use crate::platform::window::Window;

pub struct Renderer {
    vulkan: VulkanContext,
    pub width: u32,
    pub height: u32,
    pub fps: u32,
}

impl Renderer {
    pub fn new(window: &Window) -> Self {
        let vulkan = VulkanContext::new(window);

        Self {
            vulkan,
            width: 800,
            height: 600,
            fps: 0,
        }
    }

    pub fn render(&mut self, scene: &Scene) {
        self.vulkan.begin_frame();
        self.vulkan.draw_scene(scene);
        self.vulkan.end_frame();
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