use ash::vk;
use std::fs;

pub struct ShaderManager;

impl ShaderManager {
    pub fn load_shader(path: &str) -> Vec<u32> {
        let bytes = fs::read(path).unwrap_or_else(|_| {
            eprintln!("Shader not found: {}", path);
            Vec::new()
        });
        
        // Конвертируем байты в SPIR-V (u32)
        let code = bytes.chunks_exact(4)
            .map(|c| u32::from_le_bytes(c.try_into().unwrap()))
            .collect::<Vec<_>>();
        
        code
    }

    pub fn create_shader_module(device: &ash::Device, code: &[u32]) -> vk::ShaderModule {
        unsafe {
            device.create_shader_module(&vk::ShaderModuleCreateInfo::default().code(code), None)
                .expect("Failed to create shader module")
        }
    }
}