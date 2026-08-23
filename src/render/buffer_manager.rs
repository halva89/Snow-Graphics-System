use ash::vk;
use crate::render::mesh::Mesh;

pub struct BufferManager {
    pub vertex_buffer: vk::Buffer,
    pub vertex_memory: vk::DeviceMemory,
    pub index_buffer: vk::Buffer,
    pub index_memory: vk::DeviceMemory,
    pub vertex_count: u32,
    pub index_count: u32,
}

impl BufferManager {
    pub fn new() -> Self {
        Self {
            vertex_buffer: vk::Buffer::null(),
            vertex_memory: vk::DeviceMemory::null(),
            index_buffer: vk::Buffer::null(),
            index_memory: vk::DeviceMemory::null(),
            vertex_count: 0,
            index_count: 0,
        }
    }

    pub fn create_mesh_buffers(
        &mut self,
        _mesh: &Mesh,
        _device: &ash::Device,
        _physical_device: vk::PhysicalDevice,
        _instance: &ash::Instance,
    ) {
    }

    pub fn cleanup(&self, _device: &ash::Device) {}
}

impl Default for BufferManager {
    fn default() -> Self {
        Self::new()
    }
}