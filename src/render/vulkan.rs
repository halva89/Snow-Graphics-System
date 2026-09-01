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
    pub use_perspective: bool,
    pub depth_format: vk::Format,
    pub depth_image: vk::Image,
    pub depth_image_memory: vk::DeviceMemory,
    pub depth_image_view: vk::ImageView,
    current_image_index: u32,
    window_width: u32,
    window_height: u32,
}

impl VulkanContext {
    pub fn new(window: &CustomWindow, prefer_discrete: bool) -> Self {
        println!("[Vulkan] Starting initialization...");

        let entry = unsafe { Entry::load() }.expect("Failed to load Vulkan");

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

        let instance = unsafe { entry.create_instance(&instance_create_info, None) }.unwrap();

        let surface_loader = surface::Instance::new(&entry, &instance);
        let surface = unsafe {
            ash_window::create_surface(&entry, &instance, display_handle, window.get_raw_window_handle(), None)
        }.unwrap();

        let physical_devices = unsafe { instance.enumerate_physical_devices() }.unwrap();
        let has_discrete = physical_devices.iter().any(|&d| {
            unsafe { instance.get_physical_device_properties(d) }.device_type == vk::PhysicalDeviceType::DISCRETE_GPU
        });
        let has_integrated = physical_devices.iter().any(|&d| {
            unsafe { instance.get_physical_device_properties(d) }.device_type == vk::PhysicalDeviceType::INTEGRATED_GPU
        });

        let physical_device = if prefer_discrete && has_discrete {
            // Heavy mode: prefer discrete GPU
            physical_devices.iter()
                .find(|&&d| unsafe { instance.get_physical_device_properties(d) }.device_type == vk::PhysicalDeviceType::DISCRETE_GPU)
                .copied()
                .unwrap_or(physical_devices[0])
        } else if !prefer_discrete && has_integrated {
            // Light mode: prefer integrated GPU
            physical_devices.iter()
                .find(|&&d| unsafe { instance.get_physical_device_properties(d) }.device_type == vk::PhysicalDeviceType::INTEGRATED_GPU)
                .copied()
                .unwrap_or(physical_devices[0])
        } else {
            // Fallback: first available
            physical_devices[0]
        };

        let qfi = Self::find_queue_family(&instance, physical_device, surface, &surface_loader)
            .expect("No suitable queue family");

        let dev_ext = [CString::new("VK_KHR_swapchain").unwrap()];
        let ext_ptrs: Vec<*const i8> = dev_ext.iter().map(|c| c.as_ptr()).collect();

        let qp = 1.0f32;
        let qci = vk::DeviceQueueCreateInfo::default()
            .queue_family_index(qfi)
            .queue_priorities(std::slice::from_ref(&qp));

        let dci = vk::DeviceCreateInfo::default()
            .queue_create_infos(std::slice::from_ref(&qci))
            .enabled_extension_names(&ext_ptrs);

        let device = unsafe { instance.create_device(physical_device, &dci, None) }.unwrap();
        let queue = unsafe { device.get_device_queue(qfi, 0) };

        let sl = swapchain::Device::new(&instance, &device);
        let (swapchain, fmt, extent, images) =
            Self::create_swapchain(physical_device, &sl, surface, &surface_loader, window);

        let image_views: Vec<_> = images.iter().map(|&img| {
            unsafe {
                device.create_image_view(
                    &vk::ImageViewCreateInfo::default()
                        .image(img).view_type(vk::ImageViewType::TYPE_2D).format(fmt)
                        .subresource_range(vk::ImageSubresourceRange::default()
                            .aspect_mask(vk::ImageAspectFlags::COLOR)
                            .base_mip_level(0).level_count(1)
                            .base_array_layer(0).layer_count(1)),
                    None,
                )
            }.unwrap()
        }).collect();

        let (dimg, dimg_mem, dimg_view, dfmt) =
            Self::create_depth(&device, &instance, physical_device, extent.width, extent.height);

        let rp = Self::create_render_pass(&device, fmt, dfmt);

        let (dsl, dpool, dset) = Self::create_descriptors(&device);

        let pll = unsafe {
            device.create_pipeline_layout(
                &vk::PipelineLayoutCreateInfo::default().set_layouts(std::slice::from_ref(&dsl)),
                None,
            )
        }.unwrap();

        let pipeline = Pipeline::new_with_layout(&device, rp, extent, pll);

        let fbs: Vec<_> = image_views.iter().map(|&view| {
            let att = [view, dimg_view];
            unsafe {
                device.create_framebuffer(
                    &vk::FramebufferCreateInfo::default()
                        .render_pass(rp).attachments(&att)
                        .width(extent.width).height(extent.height).layers(1),
                    None,
                )
            }.unwrap()
        }).collect();

        let cpool = unsafe {
            device.create_command_pool(
                &vk::CommandPoolCreateInfo::default().queue_family_index(qfi),
                None,
            )
        }.unwrap();

        let cbs = unsafe {
            device.allocate_command_buffers(
                &vk::CommandBufferAllocateInfo::default()
                    .command_pool(cpool).level(vk::CommandBufferLevel::PRIMARY)
                    .command_buffer_count(fbs.len() as u32),
            )
        }.unwrap();

        let dummies: Vec<f32> = vec![0.0; 6];
        let dummyi: Vec<u32> = vec![0, 1, 2];
        let (vb, vbm, vc) = Self::create_vb(&device, &instance, physical_device, &dummies);
        let (ib, ibm, ic) = Self::create_ib(&device, &instance, physical_device, &dummyi);

        let mut ubo = Uniforms::new();
        let aspect = extent.width as f32 / extent.height as f32;
        ubo.projection = Mat4::perspective(45.0_f32.to_radians(), aspect, 0.1, 100.0);
        ubo.view = Mat4::identity();
        ubo.model = Mat4::identity();

        let (ub, ubm) = Self::create_ub(&device, &instance, physical_device, &ubo);

        unsafe {
            let bi = vk::DescriptorBufferInfo::default()
                .buffer(ub).offset(0).range(std::mem::size_of::<Uniforms>() as u64);
            let w = vk::WriteDescriptorSet::default()
                .dst_set(dset).dst_binding(0).dst_array_element(0)
                .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
                .buffer_info(std::slice::from_ref(&bi));
            device.update_descriptor_sets(std::slice::from_ref(&w), &[]);
        }

        let sem = vk::SemaphoreCreateInfo::default();
        let fi = vk::FenceCreateInfo::default().flags(vk::FenceCreateFlags::SIGNALED);
        let ias = unsafe { device.create_semaphore(&sem, None) }.unwrap();
        let rfs = unsafe { device.create_semaphore(&sem, None) }.unwrap();
        let fen = unsafe { device.create_fence(&fi, None) }.unwrap();

        Self {
            entry, instance, surface, surface_loader, device, physical_device,
            queue_family_index: qfi, queue,
            swapchain_loader: sl, swapchain, swapchain_images: images,
            swapchain_image_views: image_views, swapchain_format: fmt,
            swapchain_extent: extent, render_pass: rp,
            pipeline, pipeline_layout: pll,
            descriptor_set_layout: dsl, descriptor_pool: dpool, descriptor_set: dset,
            framebuffers: fbs, command_pool: cpool, command_buffers: cbs,
            image_available_semaphore: ias, render_finished_semaphore: rfs,
            fence: fen,
            vertex_buffer: vb, vertex_buffer_memory: vbm, index_buffer: ib,
            index_buffer_memory: ibm, uniform_buffer: ub, uniform_buffer_memory: ubm,
            vertex_count: vc, index_count: ic,
            current_buffer_size: 0, current_index_buffer_size: 0,
            uniforms: ubo,
            eye: Vec3::new(0.0, 0.0, 5.0), target: Vec3::zero(),
            use_perspective: true,
            depth_format: dfmt, depth_image: dimg, depth_image_memory: dimg_mem,
            depth_image_view: dimg_view,
            current_image_index: 0,
            window_width: extent.width,
            window_height: extent.height,
        }
    }

    fn find_queue_family(
        instance: &Instance, pd: vk::PhysicalDevice, surface: vk::SurfaceKHR, sl: &surface::Instance,
    ) -> Option<u32> {
        let families = unsafe { instance.get_physical_device_queue_family_properties(pd) };
        for (i, f) in families.iter().enumerate() {
            if f.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                if unsafe { sl.get_physical_device_surface_support(pd, i as u32, surface) }.unwrap_or(false) {
                    return Some(i as u32);
                }
            }
        }
        None
    }

    fn create_swapchain(
        pd: vk::PhysicalDevice, sl: &swapchain::Device,
        surface: vk::SurfaceKHR, sfl: &surface::Instance, window: &CustomWindow,
    ) -> (vk::SwapchainKHR, vk::Format, vk::Extent2D, Vec<vk::Image>) {
        let caps = unsafe { sfl.get_physical_device_surface_capabilities(pd, surface) }.unwrap();
        let formats = unsafe { sfl.get_physical_device_surface_formats(pd, surface) }.unwrap();
        let fmt = formats.iter().find(|f| {
            f.format == vk::Format::B8G8R8A8_SRGB || f.format == vk::Format::R8G8B8A8_SRGB
        }).unwrap_or(&formats[0]);
        let extent = match caps.current_extent {
            vk::Extent2D { width: u32::MAX, .. } => vk::Extent2D { width: window.width, height: window.height },
            e => e,
        };
        let mut ic = caps.min_image_count + 1;
        if caps.max_image_count > 0 && ic > caps.max_image_count { ic = caps.max_image_count; }
        let sci = vk::SwapchainCreateInfoKHR::default()
            .surface(surface).min_image_count(ic)
            .image_format(fmt.format).image_color_space(fmt.color_space)
            .image_extent(extent).image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
            .pre_transform(caps.current_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(vk::PresentModeKHR::FIFO).clipped(true);
        let sc = unsafe { sl.create_swapchain(&sci, None) }.unwrap();
        let imgs = unsafe { sl.get_swapchain_images(sc) }.unwrap();
        (sc, fmt.format, extent, imgs)
    }

    fn find_format(
        device: &Device, instance: &Instance, pd: vk::PhysicalDevice,
        candidates: &[vk::Format], tiling: vk::ImageTiling, features: vk::FormatFeatureFlags,
    ) -> Option<vk::Format> {
        for &f in candidates {
            let p = unsafe { instance.get_physical_device_format_properties(pd, f) };
            let ok = match tiling {
                vk::ImageTiling::LINEAR => p.linear_tiling_features,
                _ => p.optimal_tiling_features,
            };
            if ok.contains(features) { return Some(f); }
        }
        None
    }

    fn create_depth(
        device: &Device, instance: &Instance, pd: vk::PhysicalDevice,
        w: u32, h: u32,
    ) -> (vk::Image, vk::DeviceMemory, vk::ImageView, vk::Format) {
        let df = Self::find_format(device, instance, pd,
            &[vk::Format::D32_SFLOAT, vk::Format::D24_UNORM_S8_UINT],
            vk::ImageTiling::OPTIMAL, vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT,
        ).unwrap();
        let ii = vk::ImageCreateInfo::default()
            .image_type(vk::ImageType::TYPE_2D).format(df)
            .extent(vk::Extent3D { width: w, height: h, depth: 1 })
            .mip_levels(1).array_layers(1).samples(vk::SampleCountFlags::TYPE_1)
            .tiling(vk::ImageTiling::OPTIMAL)
            .usage(vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);
        let img = unsafe { device.create_image(&ii, None) }.unwrap();
        let mr = unsafe { device.get_image_memory_requirements(img) };
        let mt = Self::mt(instance, pd, mr.memory_type_bits, vk::MemoryPropertyFlags::DEVICE_LOCAL);
        let ai = vk::MemoryAllocateInfo::default().allocation_size(mr.size).memory_type_index(mt);
        let mem = unsafe { device.allocate_memory(&ai, None) }.unwrap();
        unsafe { device.bind_image_memory(img, mem, 0) }.unwrap();
        let vi = vk::ImageViewCreateInfo::default()
            .image(img).view_type(vk::ImageViewType::TYPE_2D).format(df)
            .subresource_range(vk::ImageSubresourceRange::default()
                .aspect_mask(vk::ImageAspectFlags::DEPTH)
                .base_mip_level(0).level_count(1)
                .base_array_layer(0).layer_count(1));
        let view = unsafe { device.create_image_view(&vi, None) }.unwrap();
        (img, mem, view, df)
    }

    fn create_render_pass(device: &Device, fmt: vk::Format, dfmt: vk::Format) -> vk::RenderPass {
        let ca = vk::AttachmentDescription::default()
            .format(fmt).samples(vk::SampleCountFlags::TYPE_1)
            .load_op(vk::AttachmentLoadOp::CLEAR).store_op(vk::AttachmentStoreOp::STORE)
            .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .final_layout(vk::ImageLayout::PRESENT_SRC_KHR);
        let da = vk::AttachmentDescription::default()
            .format(dfmt).samples(vk::SampleCountFlags::TYPE_1)
            .load_op(vk::AttachmentLoadOp::CLEAR).store_op(vk::AttachmentStoreOp::DONT_CARE)
            .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .final_layout(vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL);
        let cr = vk::AttachmentReference::default().attachment(0).layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL);
        let dr = vk::AttachmentReference::default().attachment(1).layout(vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL);
        let sp = vk::SubpassDescription::default()
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .color_attachments(std::slice::from_ref(&cr))
            .depth_stencil_attachment(&dr);
        let dep = vk::SubpassDependency::default()
            .src_subpass(vk::SUBPASS_EXTERNAL).dst_subpass(0)
            .src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT | vk::PipelineStageFlags::LATE_FRAGMENT_TESTS)
            .dst_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT | vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS)
            .dst_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE | vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE);
        let atts = [ca, da];
        unsafe {
            device.create_render_pass(
                &vk::RenderPassCreateInfo::default().attachments(&atts).subpasses(std::slice::from_ref(&sp)).dependencies(std::slice::from_ref(&dep)),
                None,
            )
        }.unwrap()
    }

    fn create_descriptors(device: &Device) -> (vk::DescriptorSetLayout, vk::DescriptorPool, vk::DescriptorSet) {
        let b = vk::DescriptorSetLayoutBinding::default()
            .binding(0).descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
            .descriptor_count(1).stage_flags(vk::ShaderStageFlags::VERTEX);
        let l = unsafe {
            device.create_descriptor_set_layout(
                &vk::DescriptorSetLayoutCreateInfo::default().bindings(std::slice::from_ref(&b)),
                None,
            )
        }.unwrap();
        let ps = vk::DescriptorPoolSize::default().ty(vk::DescriptorType::UNIFORM_BUFFER).descriptor_count(1);
        let p = unsafe {
            device.create_descriptor_pool(
                &vk::DescriptorPoolCreateInfo::default().max_sets(1).pool_sizes(std::slice::from_ref(&ps)),
                None,
            )
        }.unwrap();
        let s = unsafe {
            device.allocate_descriptor_sets(
                &vk::DescriptorSetAllocateInfo::default().descriptor_pool(p).set_layouts(std::slice::from_ref(&l)),
            )
        }.unwrap()[0];
        (l, p, s)
    }

    fn mt(
        instance: &Instance, pd: vk::PhysicalDevice, bits: u32, props: vk::MemoryPropertyFlags,
    ) -> u32 {
        let mp = unsafe { instance.get_physical_device_memory_properties(pd) };
        for i in 0..mp.memory_type_count {
            if (bits & (1 << i)) != 0 && (mp.memory_types[i as usize].property_flags & props) == props {
                return i;
            }
        }
        panic!("No memory type");
    }

    fn create_vb(
        device: &Device, instance: &Instance, pd: vk::PhysicalDevice, data: &[f32],
    ) -> (vk::Buffer, vk::DeviceMemory, u32) {
        let sz = (data.len() * 4) as u64;
        let b = unsafe { device.create_buffer(
            &vk::BufferCreateInfo::default().size(sz).usage(vk::BufferUsageFlags::VERTEX_BUFFER).sharing_mode(vk::SharingMode::EXCLUSIVE),
            None,
        ) }.unwrap();
        let mr = unsafe { device.get_buffer_memory_requirements(b) };
        let mt = Self::mt(instance, pd, mr.memory_type_bits, vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT);
        let m = unsafe { device.allocate_memory(
            &vk::MemoryAllocateInfo::default().allocation_size(mr.size).memory_type_index(mt), None,
        ) }.unwrap();
        unsafe { device.bind_buffer_memory(b, m, 0) }.unwrap();
        unsafe {
            let p = device.map_memory(m, 0, sz, vk::MemoryMapFlags::empty()).unwrap();
            std::ptr::copy_nonoverlapping(data.as_ptr() as *const u8, p as *mut u8, sz as usize);
            device.unmap_memory(m);
        }
        (b, m, (data.len() / 6) as u32)
    }

    fn create_ib(
        device: &Device, instance: &Instance, pd: vk::PhysicalDevice, data: &[u32],
    ) -> (vk::Buffer, vk::DeviceMemory, u32) {
        let sz = (data.len() * 4) as u64;
        let b = unsafe { device.create_buffer(
            &vk::BufferCreateInfo::default().size(sz).usage(vk::BufferUsageFlags::INDEX_BUFFER).sharing_mode(vk::SharingMode::EXCLUSIVE),
            None,
        ) }.unwrap();
        let mr = unsafe { device.get_buffer_memory_requirements(b) };
        let mt = Self::mt(instance, pd, mr.memory_type_bits, vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT);
        let m = unsafe { device.allocate_memory(
            &vk::MemoryAllocateInfo::default().allocation_size(mr.size).memory_type_index(mt), None,
        ) }.unwrap();
        unsafe { device.bind_buffer_memory(b, m, 0) }.unwrap();
        unsafe {
            let p = device.map_memory(m, 0, sz, vk::MemoryMapFlags::empty()).unwrap();
            std::ptr::copy_nonoverlapping(data.as_ptr() as *const u8, p as *mut u8, sz as usize);
            device.unmap_memory(m);
        }
        (b, m, data.len() as u32)
    }

    fn create_ub(
        device: &Device, instance: &Instance, pd: vk::PhysicalDevice, ubo: &Uniforms,
    ) -> (vk::Buffer, vk::DeviceMemory) {
        let sz = std::mem::size_of::<Uniforms>() as u64;
        let b = unsafe { device.create_buffer(
            &vk::BufferCreateInfo::default().size(sz).usage(vk::BufferUsageFlags::UNIFORM_BUFFER).sharing_mode(vk::SharingMode::EXCLUSIVE),
            None,
        ) }.unwrap();
        let mr = unsafe { device.get_buffer_memory_requirements(b) };
        let mt = Self::mt(instance, pd, mr.memory_type_bits, vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT);
        let m = unsafe { device.allocate_memory(
            &vk::MemoryAllocateInfo::default().allocation_size(mr.size).memory_type_index(mt), None,
        ) }.unwrap();
        unsafe { device.bind_buffer_memory(b, m, 0) }.unwrap();
        unsafe {
            let p = device.map_memory(m, 0, sz, vk::MemoryMapFlags::empty()).unwrap();
            std::ptr::copy_nonoverlapping(ubo as *const _ as *const u8, p as *mut u8, sz as usize);
            device.unmap_memory(m);
        }
        (b, m)
    }

    pub fn set_camera(&mut self, eye: Vec3, target: Vec3) {
        self.eye = eye;
        self.target = target;
    }

    fn update_ubo(&mut self) {
        let sz = std::mem::size_of::<Uniforms>() as u64;
        unsafe {
            let p = self.device.map_memory(self.uniform_buffer_memory, 0, sz, vk::MemoryMapFlags::empty()).unwrap();
            std::ptr::copy_nonoverlapping(&self.uniforms as *const _ as *const u8, p as *mut u8, sz as usize);
            self.device.unmap_memory(self.uniform_buffer_memory);
        }
    }

    fn rebuild_bufs(&mut self, verts: &[f32], idxs: &[u32]) {
        let vsz = (verts.len() * 4) as u64;
        let isz = (idxs.len() * 4) as u64;
        if vsz != self.current_buffer_size {
            unsafe {
                self.device.destroy_buffer(self.vertex_buffer, None);
                self.device.free_memory(self.vertex_buffer_memory, None);
            }
            let (vb, vm, vc) = Self::create_vb(&self.device, &self.instance, self.physical_device, verts);
            self.vertex_buffer = vb;
            self.vertex_buffer_memory = vm;
            self.vertex_count = vc;
            self.current_buffer_size = vsz;
        }
        if isz != self.current_index_buffer_size {
            unsafe {
                self.device.destroy_buffer(self.index_buffer, None);
                self.device.free_memory(self.index_buffer_memory, None);
            }
            let (ib, im, ic) = Self::create_ib(&self.device, &self.instance, self.physical_device, idxs);
            self.index_buffer = ib;
            self.index_buffer_memory = im;
            self.index_count = ic;
            self.current_index_buffer_size = isz;
        }
    }

    pub fn render_scene(&mut self, scene: &Scene, window_w: u32, window_h: u32) {
        unsafe {
            self.device.wait_for_fences(std::slice::from_ref(&self.fence), true, u64::MAX).unwrap();
            self.device.reset_fences(std::slice::from_ref(&self.fence)).unwrap();
        }

        // Try to acquire, recreate swapchain if out-of-date
        let image_index = match unsafe {
            self.swapchain_loader.acquire_next_image(self.swapchain, u64::MAX, self.image_available_semaphore, self.fence)
        } {
            Ok((idx, _)) => idx,
            Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => {
                self.rebuild_swapchain(window_w, window_h);
                return;
            }
            Err(_) => return,
        };

        // Wait for acquire to complete (fence was signaled when image was ready)
        unsafe {
            self.device.wait_for_fences(std::slice::from_ref(&self.fence), true, u64::MAX).unwrap();
            self.device.reset_fences(std::slice::from_ref(&self.fence)).unwrap();
        }

        // Build vertex data
        let mut all_v = Vec::new();
        let mut all_i = Vec::new();

        for obj in scene.get_objects() {
            let m = &obj.mesh;
            let vs = &m.vertices;
            let is = &m.indices;
            let c = obj.color.as_array();
            let base = all_v.len() / 6;
            for ch in vs.chunks(3) {
                let tv = obj.transform.model.transform(&Vec3::new(ch[0], ch[1], ch[2]));
                all_v.extend_from_slice(&[tv.x, tv.y, tv.z, c[0], c[1], c[2]]);
            }
            for &ix in is { all_i.push(ix + base as u32); }
        }

        if all_v.is_empty() || all_i.is_empty() { return; }

        let aspect = self.swapchain_extent.width as f32 / self.swapchain_extent.height as f32;
        self.uniforms.projection = Mat4::perspective(45.0_f32.to_radians(), aspect, 0.1, 100.0);
        self.uniforms.view = Mat4::look_at(self.eye, self.target, Vec3::new(0.0, 1.0, 0.0));
        self.uniforms.model = Mat4::identity();
        self.update_ubo();

        self.rebuild_bufs(&all_v, &all_i);

        // Record command buffer for this image
        let cmd = self.command_buffers[image_index as usize];
        unsafe {
            self.device.reset_command_buffer(cmd, vk::CommandBufferResetFlags::empty()).unwrap();
            self.device.begin_command_buffer(cmd, &vk::CommandBufferBeginInfo::default()).unwrap();

            let cc = vk::ClearColorValue { float32: [0.1, 0.2, 0.3, 1.0] };
            let cd = vk::ClearDepthStencilValue { depth: 1.0, stencil: 0 };
            let cv = [vk::ClearValue { color: cc }, vk::ClearValue { depth_stencil: cd }];

            let rpbi = vk::RenderPassBeginInfo::default()
                .render_pass(self.render_pass)
                .framebuffer(self.framebuffers[image_index as usize])
                .render_area(vk::Rect2D::default().offset(vk::Offset2D { x: 0, y: 0 }).extent(self.swapchain_extent))
                .clear_values(&cv);

            self.device.cmd_begin_render_pass(cmd, &rpbi, vk::SubpassContents::INLINE);
            self.device.cmd_bind_pipeline(cmd, vk::PipelineBindPoint::GRAPHICS, self.pipeline.handle);
            self.device.cmd_bind_descriptor_sets(cmd, vk::PipelineBindPoint::GRAPHICS, self.pipeline_layout,
                0, std::slice::from_ref(&self.descriptor_set), &[]);
            self.device.cmd_bind_vertex_buffers(cmd, 0, &[self.vertex_buffer], &[0]);
            self.device.cmd_bind_index_buffer(cmd, self.index_buffer, 0, vk::IndexType::UINT32);
            self.device.cmd_draw_indexed(cmd, self.index_count, 1, 0, 0, 0);
            self.device.cmd_end_render_pass(cmd);
            self.device.end_command_buffer(cmd).unwrap();
        }

        // Submit
        let si = vk::SubmitInfo::default()
            .wait_semaphores(std::slice::from_ref(&self.image_available_semaphore))
            .wait_dst_stage_mask(std::slice::from_ref(&vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT))
            .command_buffers(std::slice::from_ref(&cmd))
            .signal_semaphores(std::slice::from_ref(&self.render_finished_semaphore));

        unsafe {
            self.device.queue_submit(self.queue, std::slice::from_ref(&si), self.fence).unwrap();
        }

        // Present
        let pi = vk::PresentInfoKHR::default()
            .wait_semaphores(std::slice::from_ref(&self.render_finished_semaphore))
            .swapchains(std::slice::from_ref(&self.swapchain))
            .image_indices(std::slice::from_ref(&image_index));

        unsafe {
            let _ = self.swapchain_loader.queue_present(self.queue, &pi);
        }
    }

    fn rebuild_swapchain(&mut self, w: u32, h: u32) {
        unsafe { self.device.queue_wait_idle(self.queue).unwrap(); }
        // Destroy old
        unsafe {
            for &fb in &self.framebuffers { self.device.destroy_framebuffer(fb, None); }
            for &view in &self.swapchain_image_views { self.device.destroy_image_view(view, None); }
            self.device.destroy_image_view(self.depth_image_view, None);
            self.device.destroy_image(self.depth_image, None);
            self.device.free_memory(self.depth_image_memory, None);
            self.swapchain_loader.destroy_swapchain(self.swapchain, None);
        }

        self.window_width = w.max(1);
        self.window_height = h.max(1);

        // Recreate swapchain with a dummy window reference
        let dummy = crate::platform::CustomWindow::dummy(w, h);
        let (sc, fmt, ext, imgs) = Self::create_swapchain(
            self.physical_device, &self.swapchain_loader,
            self.surface, &self.surface_loader, &dummy,
        );
        self.swapchain = sc;
        self.swapchain_format = fmt;
        self.swapchain_extent = ext;
        self.swapchain_images = imgs;

        self.swapchain_image_views = self.swapchain_images.iter().map(|&img| {
            unsafe {
                self.device.create_image_view(
                    &vk::ImageViewCreateInfo::default()
                        .image(img).view_type(vk::ImageViewType::TYPE_2D).format(fmt)
                        .subresource_range(vk::ImageSubresourceRange::default()
                            .aspect_mask(vk::ImageAspectFlags::COLOR)
                            .base_mip_level(0).level_count(1)
                            .base_array_layer(0).layer_count(1)),
                    None,
                )
            }.unwrap()
        }).collect();

        let (dimg, dimg_mem, dimg_view, dfmt) =
            Self::create_depth(&self.device, &self.instance, self.physical_device, w, h);
        self.depth_image = dimg;
        self.depth_image_memory = dimg_mem;
        self.depth_image_view = dimg_view;
        self.depth_format = dfmt;

        self.framebuffers = self.swapchain_image_views.iter().map(|&view| {
            let att = [view, self.depth_image_view];
            unsafe {
                self.device.create_framebuffer(
                    &vk::FramebufferCreateInfo::default()
                        .render_pass(self.render_pass).attachments(&att)
                        .width(ext.width).height(ext.height).layers(1),
                    None,
                )
            }.unwrap()
        }).collect();
    }

    pub fn cleanup(&mut self) {
        unsafe {
            self.device.queue_wait_idle(self.queue).unwrap();
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
            self.device.destroy_image_view(self.depth_image_view, None);
            self.device.destroy_image(self.depth_image, None);
            self.device.free_memory(self.depth_image_memory, None);
            self.device.destroy_fence(self.fence, None);
            self.device.destroy_semaphore(self.image_available_semaphore, None);
            self.device.destroy_semaphore(self.render_finished_semaphore, None);
            self.device.destroy_command_pool(self.command_pool, None);
            self.device.destroy_render_pass(self.render_pass, None);
            for &fb in &self.framebuffers { self.device.destroy_framebuffer(fb, None); }
            for &view in &self.swapchain_image_views { self.device.destroy_image_view(view, None); }
            self.swapchain_loader.destroy_swapchain(self.swapchain, None);
            self.device.destroy_device(None);
            self.surface_loader.destroy_surface(self.surface, None);
            self.instance.destroy_instance(None);
        }
    }
}