use ash::{vk, Entry, Instance, Device};
use ash::khr::{surface, swapchain};
use crate::platform::CustomWindow;
use crate::render::Pipeline;
use crate::render::Uniforms;
use crate::core::scene::Scene;
use crate::math::{Mat4, Vec3};
use std::ffi::CString;

pub struct VulkanContext {
    pub entry: Entry,
    pub instance: Instance,
    pub surface: vk::SurfaceKHR,
    pub surface_loader: surface::Instance,
    pub device: Device,
    pub physical_device: vk::PhysicalDevice,
    pub queue_family_index: u32,
    pub queue: vk::Queue,
    pub swapchain_loader: swapchain::Device,
    pub swapchain: vk::SwapchainKHR,
    pub swapchain_images: Vec<vk::Image>,
    pub swapchain_image_views: Vec<vk::ImageView>,
    pub swapchain_format: vk::Format,
    pub swapchain_extent: vk::Extent2D,
    pub render_pass: vk::RenderPass,
    pub pipeline: Pipeline,
    pub pipeline_layout: vk::PipelineLayout,
    pub descriptor_set_layout: vk::DescriptorSetLayout,
    pub descriptor_pool: vk::DescriptorPool,
    pub descriptor_set: vk::DescriptorSet,
    pub framebuffers: Vec<vk::Framebuffer>,
    pub command_pool: vk::CommandPool,
    pub command_buffers: Vec<vk::CommandBuffer>,
    pub image_available_semaphore: vk::Semaphore,
    pub render_finished_semaphore: vk::Semaphore,
    pub fence: vk::Fence,
    pub vertex_buffer: vk::Buffer,
    pub vertex_buffer_memory: vk::DeviceMemory,
    pub index_buffer: vk::Buffer,
    pub index_buffer_memory: vk::DeviceMemory,
    pub uniform_buffer: vk::Buffer,
    pub uniform_buffer_memory: vk::DeviceMemory,
    pub vertex_count: u32,
    pub index_count: u32,
    pub current_buffer_size: u64,
    pub current_index_buffer_size: u64,
    pub uniforms: Uniforms,
    pub eye: Vec3,
    pub target: Vec3,
    current_image_index: u32,
}

impl VulkanContext {
    pub fn new(window: &CustomWindow) -> Self {
        println!("[Vulkan] Starting initialization...");

        let entry = unsafe { Entry::load() }.expect("Failed to load Vulkan");
        println!("[Vulkan] Entry loaded");

        let app_name = CString::new("Snow Graphics System").unwrap();
        let app_info = vk::ApplicationInfo::default()
            .application_name(&app_name)
            .application_version(vk::make_api_version(0, 1, 0, 0))
            .api_version(vk::make_api_version(0, 1, 0, 0));

        let display_handle = window.get_raw_display_handle();
        let extension_names = ash_window::enumerate_required_extensions(display_handle)
            .expect("Failed to get extensions")
            .to_vec();

        let instance_create_info = vk::InstanceCreateInfo::default()
            .application_info(&app_info)
            .enabled_extension_names(&extension_names);

        let instance = unsafe {
            entry.create_instance(&instance_create_info, None)
        }.expect("Failed to create Instance");
        println!("[Vulkan] Instance created");

        let surface_loader = surface::Instance::new(&entry, &instance);
        let surface = unsafe {
            ash_window::create_surface(
                &entry,
                &instance,
                display_handle,
                window.get_raw_window_handle(),
                None,
            )
        }.expect("Failed to create Surface");
        println!("[Vulkan] Surface created");

        let physical_devices = unsafe { instance.enumerate_physical_devices() }
            .expect("Failed to enumerate physical devices");
        
        let physical_device = physical_devices
            .iter()
            .find(|&&device| {
                let properties = unsafe { instance.get_physical_device_properties(device) };
                let device_type = properties.device_type;
                device_type == vk::PhysicalDeviceType::DISCRETE_GPU || 
                device_type == vk::PhysicalDeviceType::INTEGRATED_GPU
            })
            .copied()
            .unwrap_or(physical_devices[0]);
        println!("[Vulkan] Physical device selected");

        let queue_family_index = Self::find_queue_family(&instance, physical_device, surface, &surface_loader)
            .expect("Failed to find suitable queue family");
        println!("[Vulkan] Queue family found: {}", queue_family_index);

        let device_extensions = [
            CString::new("VK_KHR_swapchain").unwrap()
        ];
        
        let extension_ptrs: Vec<*const i8> = device_extensions
            .iter()
            .map(|cs| cs.as_ptr())
            .collect();

        let queue_priority = 1.0f32;
        let queue_info = vk::DeviceQueueCreateInfo::default()
            .queue_family_index(queue_family_index)
            .queue_priorities(std::slice::from_ref(&queue_priority));

        let device_info = vk::DeviceCreateInfo::default()
            .queue_create_infos(std::slice::from_ref(&queue_info))
            .enabled_extension_names(&extension_ptrs);

        let device = unsafe {
            instance.create_device(physical_device, &device_info, None)
        }.expect("Failed to create Device");
        println!("[Vulkan] Logical device created");

        let queue = unsafe { device.get_device_queue(queue_family_index, 0) };

        let swapchain_loader = swapchain::Device::new(&instance, &device);
        let (swapchain, swapchain_format, swapchain_extent, swapchain_images) = 
            Self::create_swapchain(physical_device, &swapchain_loader, surface, &surface_loader, window);
        println!("[Vulkan] Swapchain created");

        let swapchain_image_views: Vec<vk::ImageView> = swapchain_images.iter().map(|&image| {
            let view_info = vk::ImageViewCreateInfo::default()
                .image(image)
                .view_type(vk::ImageViewType::TYPE_2D)
                .format(swapchain_format)
                .subresource_range(vk::ImageSubresourceRange::default()
                    .aspect_mask(vk::ImageAspectFlags::COLOR)
                    .base_mip_level(0)
                    .level_count(1)
                    .base_array_layer(0)
                    .layer_count(1));
            unsafe { device.create_image_view(&view_info, None) }.unwrap()
        }).collect();
        println!("[Vulkan] Image views created");

        let render_pass = Self::create_render_pass(&device, swapchain_format);
        println!("[Vulkan] Render pass created");

        let (descriptor_set_layout, descriptor_pool, descriptor_set) = 
            Self::create_descriptor_sets(&device);
        println!("[Vulkan] Descriptor set created");

        let pipeline_layout_info = vk::PipelineLayoutCreateInfo::default()
            .set_layouts(std::slice::from_ref(&descriptor_set_layout));
        let pipeline_layout = unsafe { device.create_pipeline_layout(&pipeline_layout_info, None) }
            .expect("Failed to create pipeline layout");
        println!("[Vulkan] Pipeline layout created");

        let pipeline = Pipeline::new_with_layout(&device, render_pass, swapchain_extent, pipeline_layout);
        println!("[Vulkan] Pipeline created");

        let framebuffers: Vec<vk::Framebuffer> = swapchain_image_views.iter().map(|&view| {
            let attachments = [view];
            let info = vk::FramebufferCreateInfo::default()
                .render_pass(render_pass)
                .attachments(&attachments)
                .width(swapchain_extent.width)
                .height(swapchain_extent.height)
                .layers(1);
            unsafe { device.create_framebuffer(&info, None) }.unwrap()
        }).collect();
        println!("[Vulkan] Framebuffers created");

        let command_pool = unsafe {
            device.create_command_pool(
                &vk::CommandPoolCreateInfo::default()
                    .queue_family_index(queue_family_index),
                None
            )
        }.unwrap();
        println!("[Vulkan] Command pool created");

        let alloc_info = vk::CommandBufferAllocateInfo::default()
            .command_pool(command_pool)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(framebuffers.len() as u32);

        let command_buffers = unsafe { device.allocate_command_buffers(&alloc_info) }.unwrap();
        println!("[Vulkan] Command buffers allocated");

        let dummy_vertices: Vec<f32> = vec![0.0; 6];
        let dummy_indices: Vec<u32> = vec![0, 1, 2];
        let (vertex_buffer, vertex_buffer_memory, vertex_count) = Self::create_vertex_buffer(
            &device, &instance, physical_device, &dummy_vertices
        );
        let (index_buffer, index_buffer_memory, index_count) = Self::create_index_buffer(
            &device, &instance, physical_device, &dummy_indices
        );

        let mut uniforms = Uniforms::new();
        let aspect = swapchain_extent.width as f32 / swapchain_extent.height as f32;
        uniforms.projection = Mat4::orthographic(-aspect, aspect, -1.0, 1.0, 0.1, 100.0);
        uniforms.view = Mat4::identity();
        uniforms.model = Mat4::identity();

        let (uniform_buffer, uniform_buffer_memory) = Self::create_uniform_buffer(
            &device, &instance, physical_device, &uniforms
        );
        println!("[Vulkan] Uniform buffer created");

        unsafe {
            let buffer_info = vk::DescriptorBufferInfo::default()
                .buffer(uniform_buffer)
                .offset(0)
                .range(std::mem::size_of::<Uniforms>() as u64);

            let write = vk::WriteDescriptorSet::default()
                .dst_set(descriptor_set)
                .dst_binding(0)
                .dst_array_element(0)
                .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
                .buffer_info(std::slice::from_ref(&buffer_info));

            device.update_descriptor_sets(std::slice::from_ref(&write), &[]);
        }
        println!("[Vulkan] Descriptor set updated");

        let sem_info = vk::SemaphoreCreateInfo::default();
        let fence_info = vk::FenceCreateInfo::default().flags(vk::FenceCreateFlags::SIGNALED);

        let image_available_semaphore = unsafe { device.create_semaphore(&sem_info, None) }.unwrap();
        let render_finished_semaphore = unsafe { device.create_semaphore(&sem_info, None) }.unwrap();
        let fence = unsafe { device.create_fence(&fence_info, None) }.unwrap();
        println!("[Vulkan] Sync objects created");

        println!("[Vulkan] Initialization complete!");

        Self {
            entry,
            instance,
            surface,
            surface_loader,
            device,
            physical_device,
            queue_family_index,
            queue,
            swapchain_loader,
            swapchain,
            swapchain_images,
            swapchain_image_views,
            swapchain_format,
            swapchain_extent,
            render_pass,
            pipeline,
            pipeline_layout,
            descriptor_set_layout,
            descriptor_pool,
            descriptor_set,
            framebuffers,
            command_pool,
            command_buffers,
            image_available_semaphore,
            render_finished_semaphore,
            fence,
            vertex_buffer,
            vertex_buffer_memory,
            index_buffer,
            index_buffer_memory,
            uniform_buffer,
            uniform_buffer_memory,
            vertex_count,
            index_count,
            current_buffer_size: 0,
            current_index_buffer_size: 0,
            uniforms,
            eye: Vec3::new(0.0, 0.0, 3.0),
            target: Vec3::zero(),
            current_image_index: 0,
        }
    }

    fn find_queue_family(
        instance: &Instance,
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
        surface_loader: &surface::Instance,
    ) -> Option<u32> {
        let queue_families = unsafe { instance.get_physical_device_queue_family_properties(physical_device) };

        for (index, family) in queue_families.iter().enumerate() {
            if family.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                let supports_present = unsafe {
                    surface_loader.get_physical_device_surface_support(physical_device, index as u32, surface)
                }.unwrap_or(false);
                
                if supports_present {
                    return Some(index as u32);
                }
            }
        }
        None
    }

    fn create_swapchain(
        physical_device: vk::PhysicalDevice,
        swapchain_loader: &swapchain::Device,
        surface: vk::SurfaceKHR,
        surface_loader: &surface::Instance,
        window: &CustomWindow,
    ) -> (vk::SwapchainKHR, vk::Format, vk::Extent2D, Vec<vk::Image>) {
        let surface_capabilities = unsafe {
            surface_loader.get_physical_device_surface_capabilities(physical_device, surface)
        }.unwrap();

        let surface_formats = unsafe {
            surface_loader.get_physical_device_surface_formats(physical_device, surface)
        }.unwrap();

        let surface_format = surface_formats.iter()
            .find(|f| f.format == vk::Format::B8G8R8A8_SRGB || f.format == vk::Format::R8G8B8A8_SRGB)
            .unwrap_or(&surface_formats[0]);

        let extent = match surface_capabilities.current_extent {
            vk::Extent2D { width: u32::MAX, height: u32::MAX } => {
                vk::Extent2D {
                    width: window.width,
                    height: window.height,
                }
            }
            _ => surface_capabilities.current_extent,
        };

        let mut image_count = surface_capabilities.min_image_count + 1;
        if surface_capabilities.max_image_count > 0 && image_count > surface_capabilities.max_image_count {
            image_count = surface_capabilities.max_image_count;
        }

        let swapchain_info = vk::SwapchainCreateInfoKHR::default()
            .surface(surface)
            .min_image_count(image_count)
            .image_format(surface_format.format)
            .image_color_space(surface_format.color_space)
            .image_extent(extent)
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
            .pre_transform(surface_capabilities.current_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(vk::PresentModeKHR::FIFO)
            .clipped(true);

        let swapchain = unsafe {
            swapchain_loader.create_swapchain(&swapchain_info, None)
        }.expect("Failed to create Swapchain");

        let swapchain_images = unsafe {
            swapchain_loader.get_swapchain_images(swapchain)
        }.expect("Failed to get swapchain images");

        (swapchain, surface_format.format, extent, swapchain_images)
    }

    fn create_render_pass(device: &Device, format: vk::Format) -> vk::RenderPass {
        let color_attachment = vk::AttachmentDescription::default()
            .format(format)
            .samples(vk::SampleCountFlags::TYPE_1)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .final_layout(vk::ImageLayout::PRESENT_SRC_KHR);

        let color_ref = vk::AttachmentReference::default()
            .attachment(0)
            .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL);

        let subpass = vk::SubpassDescription::default()
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .color_attachments(std::slice::from_ref(&color_ref));

        let dependency = vk::SubpassDependency::default()
            .src_subpass(vk::SUBPASS_EXTERNAL)
            .dst_subpass(0)
            .src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .dst_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .src_access_mask(vk::AccessFlags::empty())
            .dst_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE);

        let render_pass_info = vk::RenderPassCreateInfo::default()
            .attachments(std::slice::from_ref(&color_attachment))
            .subpasses(std::slice::from_ref(&subpass))
            .dependencies(std::slice::from_ref(&dependency));

        unsafe { device.create_render_pass(&render_pass_info, None) }
            .expect("Failed to create Render Pass")
    }

    fn create_descriptor_sets(
        device: &Device,
    ) -> (vk::DescriptorSetLayout, vk::DescriptorPool, vk::DescriptorSet) {
        let binding = vk::DescriptorSetLayoutBinding::default()
            .binding(0)
            .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::VERTEX);

        let layout_info = vk::DescriptorSetLayoutCreateInfo::default()
            .bindings(std::slice::from_ref(&binding));

        let layout = unsafe { device.create_descriptor_set_layout(&layout_info, None) }
            .expect("Failed to create descriptor set layout");

        let pool_size = vk::DescriptorPoolSize::default()
            .ty(vk::DescriptorType::UNIFORM_BUFFER)
            .descriptor_count(1);

        let pool_info = vk::DescriptorPoolCreateInfo::default()
            .max_sets(1)
            .pool_sizes(std::slice::from_ref(&pool_size));

        let pool = unsafe { device.create_descriptor_pool(&pool_info, None) }
            .expect("Failed to create descriptor pool");

        let alloc_info = vk::DescriptorSetAllocateInfo::default()
            .descriptor_pool(pool)
            .set_layouts(std::slice::from_ref(&layout));

        let set = unsafe { device.allocate_descriptor_sets(&alloc_info) }
            .expect("Failed to allocate descriptor set")[0];

        (layout, pool, set)
    }

    fn find_memory_type(
        instance: &Instance,
        physical_device: vk::PhysicalDevice,
        memory_type_bits: u32,
        properties: vk::MemoryPropertyFlags,
    ) -> u32 {
        let memory_properties = unsafe {
            instance.get_physical_device_memory_properties(physical_device)
        };

        for i in 0..memory_properties.memory_type_count {
            if (memory_type_bits & (1 << i)) != 0
                && (memory_properties.memory_types[i as usize].property_flags & properties) == properties
            {
                return i;
            }
        }

        panic!("Failed to find suitable memory type");
    }

    fn create_vertex_buffer(
        device: &Device,
        instance: &Instance,
        physical_device: vk::PhysicalDevice,
        vertices: &[f32],
    ) -> (vk::Buffer, vk::DeviceMemory, u32) {
        let vertex_count = (vertices.len() / 6) as u32;
        let buffer_size = (vertices.len() * std::mem::size_of::<f32>()) as u64;

        let buffer_info = vk::BufferCreateInfo::default()
            .size(buffer_size)
            .usage(vk::BufferUsageFlags::VERTEX_BUFFER)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);

        let vertex_buffer = unsafe { device.create_buffer(&buffer_info, None) }
            .expect("Failed to create vertex buffer");

        let memory_requirements = unsafe { device.get_buffer_memory_requirements(vertex_buffer) };
        let memory_type_index = Self::find_memory_type(
            instance,
            physical_device,
            memory_requirements.memory_type_bits,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
        );

        let memory_info = vk::MemoryAllocateInfo::default()
            .allocation_size(memory_requirements.size)
            .memory_type_index(memory_type_index);

        let vertex_buffer_memory = unsafe { device.allocate_memory(&memory_info, None) }
            .expect("Failed to allocate vertex buffer memory");

        unsafe {
            device.bind_buffer_memory(vertex_buffer, vertex_buffer_memory, 0)
                .expect("Failed to bind buffer memory");
            
            let data_ptr = device.map_memory(vertex_buffer_memory, 0, buffer_size, vk::MemoryMapFlags::empty())
                .expect("Failed to map memory");
            
            let slice = std::slice::from_raw_parts_mut(data_ptr as *mut u8, buffer_size as usize);
            
            let vertex_bytes: &[u8] = std::slice::from_raw_parts(
                vertices.as_ptr() as *const u8, 
                buffer_size as usize
            );
            slice.copy_from_slice(vertex_bytes);
            
            device.unmap_memory(vertex_buffer_memory);
        }

        (vertex_buffer, vertex_buffer_memory, vertex_count)
    }

    fn create_index_buffer(
        device: &Device,
        instance: &Instance,
        physical_device: vk::PhysicalDevice,
        indices: &[u32],
    ) -> (vk::Buffer, vk::DeviceMemory, u32) {
        let index_count = indices.len() as u32;
        let buffer_size = (indices.len() * std::mem::size_of::<u32>()) as u64;

        let buffer_info = vk::BufferCreateInfo::default()
            .size(buffer_size)
            .usage(vk::BufferUsageFlags::INDEX_BUFFER)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);

        let index_buffer = unsafe { device.create_buffer(&buffer_info, None) }
            .expect("Failed to create index buffer");

        let memory_requirements = unsafe { device.get_buffer_memory_requirements(index_buffer) };
        let memory_type_index = Self::find_memory_type(
            instance,
            physical_device,
            memory_requirements.memory_type_bits,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
        );

        let memory_info = vk::MemoryAllocateInfo::default()
            .allocation_size(memory_requirements.size)
            .memory_type_index(memory_type_index);

        let index_buffer_memory = unsafe { device.allocate_memory(&memory_info, None) }
            .expect("Failed to allocate index buffer memory");

        unsafe {
            device.bind_buffer_memory(index_buffer, index_buffer_memory, 0)
                .expect("Failed to bind buffer memory");
            
            let data_ptr = device.map_memory(index_buffer_memory, 0, buffer_size, vk::MemoryMapFlags::empty())
                .expect("Failed to map memory");
            
            let slice = std::slice::from_raw_parts_mut(data_ptr as *mut u8, buffer_size as usize);
            
            let index_bytes: &[u8] = std::slice::from_raw_parts(
                indices.as_ptr() as *const u8, 
                buffer_size as usize
            );
            slice.copy_from_slice(index_bytes);
            
            device.unmap_memory(index_buffer_memory);
        }

        (index_buffer, index_buffer_memory, index_count)
    }

    fn create_uniform_buffer(
        device: &Device,
        instance: &Instance,
        physical_device: vk::PhysicalDevice,
        uniforms: &Uniforms,
    ) -> (vk::Buffer, vk::DeviceMemory) {
        let buffer_size = std::mem::size_of::<Uniforms>() as u64;

        let buffer_info = vk::BufferCreateInfo::default()
            .size(buffer_size)
            .usage(vk::BufferUsageFlags::UNIFORM_BUFFER)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);

        let buffer = unsafe { device.create_buffer(&buffer_info, None) }
            .expect("Failed to create uniform buffer");

        let memory_requirements = unsafe { device.get_buffer_memory_requirements(buffer) };
        let memory_type_index = Self::find_memory_type(
            instance,
            physical_device,
            memory_requirements.memory_type_bits,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
        );

        let memory_info = vk::MemoryAllocateInfo::default()
            .allocation_size(memory_requirements.size)
            .memory_type_index(memory_type_index);

        let memory = unsafe { device.allocate_memory(&memory_info, None) }
            .expect("Failed to allocate uniform buffer memory");

        unsafe {
            device.bind_buffer_memory(buffer, memory, 0)
                .expect("Failed to bind uniform buffer memory");
            
            let data_ptr = device.map_memory(memory, 0, buffer_size, vk::MemoryMapFlags::empty())
                .expect("Failed to map uniform memory");
            
            let slice = std::slice::from_raw_parts_mut(data_ptr as *mut u8, buffer_size as usize);
            let uniform_bytes: &[u8] = std::slice::from_raw_parts(
                uniforms as *const _ as *const u8,
                buffer_size as usize
            );
            slice.copy_from_slice(uniform_bytes);
            device.unmap_memory(memory);
        }

        (buffer, memory)
    }

    pub fn set_camera(&mut self, eye: Vec3, target: Vec3) {
        self.eye = eye;
        self.target = target;
    }

    fn update_uniform_buffer(&mut self) {
        let buffer_size = std::mem::size_of::<Uniforms>() as u64;
        unsafe {
            let data_ptr = self.device.map_memory(self.uniform_buffer_memory, 0, buffer_size, vk::MemoryMapFlags::empty())
                .expect("Failed to map uniform memory");
            
            let slice = std::slice::from_raw_parts_mut(data_ptr as *mut u8, buffer_size as usize);
            let uniform_bytes: &[u8] = std::slice::from_raw_parts(
                &self.uniforms as *const _ as *const u8,
                buffer_size as usize
            );
            slice.copy_from_slice(uniform_bytes);
            self.device.unmap_memory(self.uniform_buffer_memory);
        }
    }

    pub fn render_scene(&mut self, scene: &Scene) {
        println!("[Render] render_scene called");
        
        if !self.begin_frame() {
            println!("[Render] begin_frame failed");
            return;
        }

        let mut all_vertices = Vec::new();
        let mut all_indices = Vec::new();

        for object in scene.get_objects() {
            let mesh = &object.mesh;
            let vertices = &mesh.vertices;
            let indices = &mesh.indices;
            let color = object.color.as_array();
            
            let start_idx = all_vertices.len() / 6;
            for chunk in vertices.chunks(3) {
                all_vertices.push(chunk[0]);
                all_vertices.push(chunk[1]);
                all_vertices.push(chunk[2]);
                all_vertices.push(color[0]);
                all_vertices.push(color[1]);
                all_vertices.push(color[2]);
            }
            
            for &idx in indices {
                all_indices.push(idx + start_idx as u32);
            }
        }

        if all_vertices.is_empty() || all_indices.is_empty() {
            println!("[Render] No vertices or indices");
            self.end_frame();
            return;
        }

        // Обновляем uniform
        let aspect = self.swapchain_extent.width as f32 / self.swapchain_extent.height as f32;
        self.uniforms.projection = Mat4::orthographic(-aspect, aspect, -1.0, 1.0, 0.1, 100.0);
        let up = Vec3::new(0.0, 1.0, 0.0);
        self.uniforms.view = Mat4::look_at(self.eye, self.target, up);
        self.uniforms.model = Mat4::identity();
        self.update_uniform_buffer();

        // Обновляем вершинный буфер (только если изменился размер)
        let vertex_buffer_size = (all_vertices.len() * std::mem::size_of::<f32>()) as u64;
        if vertex_buffer_size != self.current_buffer_size {
            println!("[Render] Recreating vertex buffer, size: {}", vertex_buffer_size);
            
            unsafe {
                self.device.destroy_buffer(self.vertex_buffer, None);
                self.device.free_memory(self.vertex_buffer_memory, None);
            }

            let buffer_info = vk::BufferCreateInfo::default()
                .size(vertex_buffer_size)
                .usage(vk::BufferUsageFlags::VERTEX_BUFFER)
                .sharing_mode(vk::SharingMode::EXCLUSIVE);

            self.vertex_buffer = unsafe { self.device.create_buffer(&buffer_info, None) }
                .expect("Failed to create vertex buffer");

            let memory_requirements = unsafe { self.device.get_buffer_memory_requirements(self.vertex_buffer) };
            let memory_type_index = Self::find_memory_type(
                &self.instance,
                self.physical_device,
                memory_requirements.memory_type_bits,
                vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            );

            let memory_info = vk::MemoryAllocateInfo::default()
                .allocation_size(memory_requirements.size)
                .memory_type_index(memory_type_index);

            self.vertex_buffer_memory = unsafe { self.device.allocate_memory(&memory_info, None) }
                .expect("Failed to allocate vertex buffer memory");

            unsafe {
                self.device.bind_buffer_memory(self.vertex_buffer, self.vertex_buffer_memory, 0)
                    .expect("Failed to bind buffer memory");
            }
            
            self.current_buffer_size = vertex_buffer_size;
            self.vertex_count = (all_vertices.len() / 6) as u32;
        }

        // Обновляем индексный буфер (только если изменился размер)
        let index_buffer_size = (all_indices.len() * std::mem::size_of::<u32>()) as u64;
        if index_buffer_size != self.current_index_buffer_size {
            println!("[Render] Recreating index buffer, size: {}", index_buffer_size);
            
            unsafe {
                self.device.destroy_buffer(self.index_buffer, None);
                self.device.free_memory(self.index_buffer_memory, None);
            }

            let buffer_info = vk::BufferCreateInfo::default()
                .size(index_buffer_size)
                .usage(vk::BufferUsageFlags::INDEX_BUFFER)
                .sharing_mode(vk::SharingMode::EXCLUSIVE);

            self.index_buffer = unsafe { self.device.create_buffer(&buffer_info, None) }
                .expect("Failed to create index buffer");

            let memory_requirements = unsafe { self.device.get_buffer_memory_requirements(self.index_buffer) };
            let memory_type_index = Self::find_memory_type(
                &self.instance,
                self.physical_device,
                memory_requirements.memory_type_bits,
                vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            );

            let memory_info = vk::MemoryAllocateInfo::default()
                .allocation_size(memory_requirements.size)
                .memory_type_index(memory_type_index);

            self.index_buffer_memory = unsafe { self.device.allocate_memory(&memory_info, None) }
                .expect("Failed to allocate index buffer memory");

            unsafe {
                self.device.bind_buffer_memory(self.index_buffer, self.index_buffer_memory, 0)
                    .expect("Failed to bind buffer memory");
            }
            
            self.current_index_buffer_size = index_buffer_size;
            self.index_count = all_indices.len() as u32;
        }

        // Копируем данные в буферы (всегда)
        unsafe {
            let data_ptr = self.device.map_memory(self.vertex_buffer_memory, 0, vertex_buffer_size, vk::MemoryMapFlags::empty())
                .expect("Failed to map vertex memory");
            
            let slice = std::slice::from_raw_parts_mut(data_ptr as *mut u8, vertex_buffer_size as usize);
            let vertex_bytes: &[u8] = std::slice::from_raw_parts(
                all_vertices.as_ptr() as *const u8, 
                vertex_buffer_size as usize
            );
            slice.copy_from_slice(vertex_bytes);
            self.device.unmap_memory(self.vertex_buffer_memory);
        }

        unsafe {
            let data_ptr = self.device.map_memory(self.index_buffer_memory, 0, index_buffer_size, vk::MemoryMapFlags::empty())
                .expect("Failed to map index memory");
            
            let slice = std::slice::from_raw_parts_mut(data_ptr as *mut u8, index_buffer_size as usize);
            let index_bytes: &[u8] = std::slice::from_raw_parts(
                all_indices.as_ptr() as *const u8, 
                index_buffer_size as usize
            );
            slice.copy_from_slice(index_bytes);
            self.device.unmap_memory(self.index_buffer_memory);
        }

        // Перезаписываем command buffer
        let current_index = self.current_image_index as usize;
        let cmd = self.command_buffers[current_index];
        
        unsafe {
            self.device.reset_command_buffer(cmd, vk::CommandBufferResetFlags::empty()).unwrap();
            self.device.begin_command_buffer(cmd, &vk::CommandBufferBeginInfo::default()).unwrap();

            let clear_color = vk::ClearColorValue {
                float32: [0.1, 0.2, 0.8, 1.0],
            };
            let clear_values = [vk::ClearValue { color: clear_color }];

            let render_pass_begin = vk::RenderPassBeginInfo::default()
                .render_pass(self.render_pass)
                .framebuffer(self.framebuffers[current_index])
                .render_area(vk::Rect2D::default()
                    .offset(vk::Offset2D { x: 0, y: 0 })
                    .extent(self.swapchain_extent))
                .clear_values(&clear_values);

            self.device.cmd_begin_render_pass(cmd, &render_pass_begin, vk::SubpassContents::INLINE);
            self.device.cmd_bind_pipeline(cmd, vk::PipelineBindPoint::GRAPHICS, self.pipeline.handle);
            
            self.device.cmd_bind_descriptor_sets(
                cmd,
                vk::PipelineBindPoint::GRAPHICS,
                self.pipeline_layout,
                0,
                std::slice::from_ref(&self.descriptor_set),
                &[],
            );
            
            let vertex_buffers = [self.vertex_buffer];
            let offsets = [0];
            self.device.cmd_bind_vertex_buffers(cmd, 0, &vertex_buffers, &offsets);
            
            self.device.cmd_bind_index_buffer(cmd, self.index_buffer, 0, vk::IndexType::UINT32);
            
            self.device.cmd_draw_indexed(cmd, self.index_count, 1, 0, 0, 0);
            
            self.device.cmd_end_render_pass(cmd);
            self.device.end_command_buffer(cmd).unwrap();
        }

        self.end_frame();
    }

    fn begin_frame(&mut self) -> bool {
        unsafe {
            let (image_index, _) = match self.swapchain_loader.acquire_next_image(
                self.swapchain,
                u64::MAX,
                self.image_available_semaphore,
                vk::Fence::null(),
            ) {
                Ok(result) => result,
                Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => return false,
                Err(e) => panic!("Failed to acquire next image: {:?}", e),
            };

            self.current_image_index = image_index;
            let cmd = self.command_buffers[image_index as usize];

            let submit_info = vk::SubmitInfo::default()
                .wait_semaphores(std::slice::from_ref(&self.image_available_semaphore))
                .wait_dst_stage_mask(std::slice::from_ref(&vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT))
                .command_buffers(std::slice::from_ref(&cmd))
                .signal_semaphores(std::slice::from_ref(&self.render_finished_semaphore));

            self.device.queue_submit(self.queue, std::slice::from_ref(&submit_info), self.fence)
                .expect("Failed to submit command buffer");

            true
        }
    }

    fn end_frame(&mut self) {
        let current_index = self.current_image_index;
        let present_info = vk::PresentInfoKHR::default()
            .wait_semaphores(std::slice::from_ref(&self.render_finished_semaphore))
            .swapchains(std::slice::from_ref(&self.swapchain))
            .image_indices(std::slice::from_ref(&current_index));

        unsafe {
            let _ = self.swapchain_loader.queue_present(self.queue, &present_info);
        }
    }

    pub fn cleanup(&mut self) {
        unsafe {
            self.pipeline.cleanup(&self.device);
            self.device.destroy_pipeline_layout(self.pipeline_layout, None);
            
            self.device.destroy_descriptor_pool(self.descriptor_pool, None);
            self.device.destroy_descriptor_set_layout(self.descriptor_set_layout, None);
            
            self.device.destroy_buffer(self.vertex_buffer, None);
            self.device.free_memory(self.vertex_buffer_memory, None);
            self.device.destroy_buffer(self.index_buffer, None);
            self.device.free_memory(self.index_buffer_memory, None);
            self.device.destroy_buffer(self.uniform_buffer, None);
            self.device.free_memory(self.uniform_buffer_memory, None);
            
            self.device.destroy_fence(self.fence, None);
            self.device.destroy_semaphore(self.image_available_semaphore, None);
            self.device.destroy_semaphore(self.render_finished_semaphore, None);
            self.device.destroy_command_pool(self.command_pool, None);
            self.device.destroy_render_pass(self.render_pass, None);
            
            for &fb in &self.framebuffers {
                self.device.destroy_framebuffer(fb, None);
            }
            
            for &view in &self.swapchain_image_views {
                self.device.destroy_image_view(view, None);
            }
            
            self.swapchain_loader.destroy_swapchain(self.swapchain, None);
            self.device.destroy_device(None);
            self.surface_loader.destroy_surface(self.surface, None);
            self.instance.destroy_instance(None);
        }
    }
}