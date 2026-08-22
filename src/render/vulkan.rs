use ash::{vk, Entry, Instance, Device};
use ash::khr::{surface, swapchain};
use crate::platform::window::Window;
use std::ffi::CString;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

pub struct VulkanContext {
    pub _entry: Entry,
    pub _instance: Instance,
    pub _surface_loader: surface::Instance,
    pub _surface: vk::SurfaceKHR,
    pub _physical_device: vk::PhysicalDevice,
    pub _device: Device,
    pub _queue: vk::Queue,
    pub _swapchain_loader: swapchain::Device,
}

impl VulkanContext {
    pub fn new(window: &Window) -> Self {
        let entry = unsafe { Entry::load() }.expect("Failed to load Vulkan");

        let app_name = CString::new("Snow Graphics System").unwrap();
        let app_info = vk::ApplicationInfo::default()
            .application_name(&app_name)
            .application_version(vk::make_api_version(0, 1, 0, 0))
            .api_version(vk::make_api_version(0, 1, 0, 0));

        let instance = unsafe {
            entry.create_instance(&vk::InstanceCreateInfo::default().application_info(&app_info), None)
        }.expect("Failed to create Instance");

        let surface_loader = surface::Instance::new(&entry, &instance);

        let surface = unsafe {
            ash_window::create_surface(
                &entry,
                &instance,
                window.raw.display_handle().unwrap().as_raw(),
                window.raw.window_handle().unwrap().as_raw(),
                None,
            )
        }.expect("Failed to create Surface");

        let devices = unsafe { instance.enumerate_physical_devices() }.unwrap();
        let physical_device = devices[0];

        let device = unsafe {
            instance.create_device(physical_device, &vk::DeviceCreateInfo::default(), None)
        }.expect("Failed to create Device");
        let queue = unsafe { device.get_device_queue(0, 0) };
        let swapchain_loader = swapchain::Device::new(&instance, &device);

        Self {
            _entry: entry,
            _instance: instance,
            _surface: surface,
            _surface_loader: surface_loader,
            _physical_device: physical_device,
            _device: device,
            _queue: queue,
            _swapchain_loader: swapchain_loader,
        }
    }

    // Заглушки для совместимости
    pub fn begin_frame(&mut self) {}
    pub fn draw_scene(&mut self, _scene: &crate::core::scene::Scene) {}
    pub fn end_frame(&mut self) {}
    pub fn cleanup(&mut self) {}
}