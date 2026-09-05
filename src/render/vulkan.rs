use ash::{vk, Entry, Instance, Device};
use ash::khr::{surface, swapchain};
use crate::platform::CustomWindow;
use crate::render::Pipeline;
use crate::render::Uniforms;
use crate::core::scene::Scene;
use crate::math::{Mat4, Vec3};
use std::ffi::CString;

pub struct VulkanContext {
    pub entry: Entry, pub instance: Instance,
    pub surface: vk::SurfaceKHR, pub surface_loader: surface::Instance,
    pub device: Device, pub physical_device: vk::PhysicalDevice,
    pub queue_family_index: u32, pub queue: vk::Queue,
    pub swapchain_loader: swapchain::Device,
    pub swapchain: vk::SwapchainKHR,
    pub swapchain_images: Vec<vk::Image>,
    pub swapchain_image_views: Vec<vk::ImageView>,
    pub swapchain_format: vk::Format,
    pub swapchain_extent: vk::Extent2D,
    pub render_pass: vk::RenderPass,
    pub pipeline: Pipeline, pub pipeline_layout: vk::PipelineLayout,
    pub descriptor_set_layout: vk::DescriptorSetLayout,
    pub descriptor_pool: vk::DescriptorPool,
    pub descriptor_set_scene: vk::DescriptorSet,
    pub descriptor_set_ov: vk::DescriptorSet,
    pub framebuffers: Vec<vk::Framebuffer>,
    pub command_pool: vk::CommandPool,
    pub command_buffers: Vec<vk::CommandBuffer>,
    pub image_available_semaphore: vk::Semaphore,
    pub render_finished_semaphore: vk::Semaphore, pub fence: vk::Fence,
    pub vertex_buffer: vk::Buffer, pub vertex_buffer_memory: vk::DeviceMemory,
    pub index_buffer: vk::Buffer, pub index_buffer_memory: vk::DeviceMemory,
    pub edge_vertex_buffer: vk::Buffer, pub edge_vertex_buffer_memory: vk::DeviceMemory,
    pub edge_index_buffer: vk::Buffer, pub edge_index_buffer_memory: vk::DeviceMemory,
    pub uniform_buffer: vk::Buffer, pub uniform_buffer_memory: vk::DeviceMemory,
    pub uniform_buffer_ov: vk::Buffer, pub uniform_buffer_ov_memory: vk::DeviceMemory,
    pub vertex_count: u32, pub index_count: u32,
    pub edge_vertex_count: u32, pub edge_index_count: u32,
    pub current_buffer_size: u64, pub current_index_buffer_size: u64,
    pub current_edge_buffer_size: u64, pub current_edge_index_buffer_size: u64,
    pub line_width: f32,
    pub uniforms: Uniforms, pub uniforms_ov: Uniforms,
    pub eye: Vec3, pub target: Vec3,
    pub depth_format: vk::Format,
    pub depth_image: vk::Image, pub depth_image_memory: vk::DeviceMemory,
    pub depth_image_view: vk::ImageView,
    current_image_index: u32, window_width: u32, window_height: u32, wireframe: bool,
}

impl VulkanContext {
    pub fn new(window: &CustomWindow, prefer_discrete: bool) -> Self {
        println!("[Vulkan] Starting init...");
        let entry = unsafe { Entry::load() }.unwrap();
        let app_name = CString::new("Snow").unwrap();
        let app_info = vk::ApplicationInfo::default()
            .application_name(&app_name).application_version(1).api_version(vk::make_api_version(0, 1, 0, 0));
        let dh = window.get_raw_display_handle();
        let exts = ash_window::enumerate_required_extensions(dh).unwrap().to_vec();
        let ici = vk::InstanceCreateInfo::default().application_info(&app_info).enabled_extension_names(&exts);
        let instance = unsafe { entry.create_instance(&ici, None) }.unwrap();
        let sfl = surface::Instance::new(&entry, &instance);
        let surface = unsafe { ash_window::create_surface(&entry, &instance, dh, window.get_raw_window_handle(), None) }.unwrap();
        let pds = unsafe { instance.enumerate_physical_devices() }.unwrap();
        let has_discrete = pds.iter().any(|&d| unsafe { instance.get_physical_device_properties(d) }.device_type == vk::PhysicalDeviceType::DISCRETE_GPU);
        let has_integrated = pds.iter().any(|&d| unsafe { instance.get_physical_device_properties(d) }.device_type == vk::PhysicalDeviceType::INTEGRATED_GPU);
        let pd = if prefer_discrete && has_discrete {
            *pds.iter().find(|&&d| unsafe { instance.get_physical_device_properties(d) }.device_type == vk::PhysicalDeviceType::DISCRETE_GPU).unwrap_or(&pds[0])
        } else if !prefer_discrete && has_integrated {
            *pds.iter().find(|&&d| unsafe { instance.get_physical_device_properties(d) }.device_type == vk::PhysicalDeviceType::INTEGRATED_GPU).unwrap_or(&pds[0])
        } else { pds[0] };
        let qfi = Self::find_qf(&instance, pd, surface, &sfl).unwrap();
        let dev_ext = [CString::new("VK_KHR_swapchain").unwrap()];
        let ext_ptrs: Vec<*const i8> = dev_ext.iter().map(|c| c.as_ptr()).collect();
        let qp = 1.0f32;
        let qci = vk::DeviceQueueCreateInfo::default().queue_family_index(qfi).queue_priorities(std::slice::from_ref(&qp));
        let mut features = vk::PhysicalDeviceFeatures::default();
        features.wide_lines = vk::TRUE;
        let dci = vk::DeviceCreateInfo::default().queue_create_infos(std::slice::from_ref(&qci)).enabled_extension_names(&ext_ptrs).enabled_features(&features);
        let device = unsafe { instance.create_device(pd, &dci, None) }.unwrap();
        let queue = unsafe { device.get_device_queue(qfi, 0) };
        let sl = swapchain::Device::new(&instance, &device);
        let (sc, fmt, ext, imgs) = Self::create_sc(pd, &sl, surface, &sfl, window.width, window.height)
            .expect("Failed to create swapchain: surface is not available (minimized?)");
        let ivs: Vec<_> = imgs.iter().map(|&img| unsafe { device.create_image_view(
            &vk::ImageViewCreateInfo::default().image(img).view_type(vk::ImageViewType::TYPE_2D).format(fmt)
            .subresource_range(vk::ImageSubresourceRange::default().aspect_mask(vk::ImageAspectFlags::COLOR).base_mip_level(0).level_count(1).base_array_layer(0).layer_count(1)), None) }.unwrap()).collect();
        let (dimg, dimg_mem, dimg_view, dfmt) = Self::create_depth(&device, &instance, pd, ext.width, ext.height);
        let rp = Self::create_rp(&device, fmt, dfmt);
        let (dsl, dpool, dset, dset2) = Self::create_desc(&device);
        let pll = unsafe { device.create_pipeline_layout(&vk::PipelineLayoutCreateInfo::default().set_layouts(std::slice::from_ref(&dsl)), None) }.unwrap();
        let pipeline = Pipeline::new_with_layout(&device, rp, ext, pll, 2.0);
        let fbs: Vec<_> = ivs.iter().map(|&view| {
            let att = [view, dimg_view];
            unsafe { device.create_framebuffer(&vk::FramebufferCreateInfo::default().render_pass(rp).attachments(&att).width(ext.width).height(ext.height).layers(1), None) }.unwrap()
        }).collect();
        let cpool = unsafe { device.create_command_pool(&vk::CommandPoolCreateInfo::default().queue_family_index(qfi), None) }.unwrap();
        let cbs = unsafe { device.allocate_command_buffers(&vk::CommandBufferAllocateInfo::default().command_pool(cpool).level(vk::CommandBufferLevel::PRIMARY).command_buffer_count(fbs.len() as u32)) }.unwrap();
        let dv: Vec<f32> = vec![0.0; 6];
        let di: Vec<u32> = vec![0, 1, 2];
        let (vb, vbm, vc) = Self::create_vb(&device, &instance, pd, &dv);
        let (ib, ibm, ic) = Self::create_ib(&device, &instance, pd, &di);
        let aspect = ext.width as f32 / ext.height as f32;
        let mut ubo = Uniforms::new();
        ubo.projection = Mat4::perspective(45.0_f32.to_radians(), aspect, 0.1, 100.0);
        let (ub, ubm) = Self::create_ub(&device, &instance, pd, &ubo);
        let mut ubo_ov = Uniforms::new();
        ubo_ov.projection = Mat4::orthographic(-1.0, 1.0, -1.0, 1.0, 0.0, 10.0);
        let (ub2, ubm2) = Self::create_ub(&device, &instance, pd, &ubo_ov);
        unsafe {
            let bi = vk::DescriptorBufferInfo::default().buffer(ub).offset(0).range(std::mem::size_of::<Uniforms>() as u64);
            let w = vk::WriteDescriptorSet::default().dst_set(dset).dst_binding(0).descriptor_type(vk::DescriptorType::UNIFORM_BUFFER).buffer_info(std::slice::from_ref(&bi));
            device.update_descriptor_sets(std::slice::from_ref(&w), &[]);
            let bi2 = vk::DescriptorBufferInfo::default().buffer(ub2).offset(0).range(std::mem::size_of::<Uniforms>() as u64);
            let w2 = vk::WriteDescriptorSet::default().dst_set(dset2).dst_binding(0).descriptor_type(vk::DescriptorType::UNIFORM_BUFFER).buffer_info(std::slice::from_ref(&bi2));
            device.update_descriptor_sets(std::slice::from_ref(&w2), &[]);
        }
        let sem = vk::SemaphoreCreateInfo::default();
        let fi = vk::FenceCreateInfo::default().flags(vk::FenceCreateFlags::SIGNALED);
        let ias = unsafe { device.create_semaphore(&sem, None) }.unwrap();
        let rfs = unsafe { device.create_semaphore(&sem, None) }.unwrap();
        let fen = unsafe { device.create_fence(&fi, None) }.unwrap();
        // Edge-буферы: фиктивные (1 вершина/1 индекс), т.к. Vulkan запрещает размер 0
        let (evb, evbm, evc) = Self::create_vb(&device, &instance, pd, &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        let (eib, eibm, eic) = Self::create_ib(&device, &instance, pd, &[0]);
        Self {
            entry, instance, surface, surface_loader: sfl, device, physical_device: pd,
            queue_family_index: qfi, queue, swapchain_loader: sl, swapchain: sc,
            swapchain_images: imgs, swapchain_image_views: ivs, swapchain_format: fmt,
            swapchain_extent: ext, render_pass: rp, pipeline, pipeline_layout: pll,
            descriptor_set_layout: dsl, descriptor_pool: dpool,
            descriptor_set_scene: dset, descriptor_set_ov: dset2,
            framebuffers: fbs, command_pool: cpool, command_buffers: cbs,
            image_available_semaphore: ias, render_finished_semaphore: rfs, fence: fen,
            vertex_buffer: vb, vertex_buffer_memory: vbm, index_buffer: ib,
            index_buffer_memory: ibm, edge_vertex_buffer: evb, edge_vertex_buffer_memory: evbm,
            edge_index_buffer: eib, edge_index_buffer_memory: eibm,
            uniform_buffer: ub, uniform_buffer_memory: ubm,
            uniform_buffer_ov: ub2, uniform_buffer_ov_memory: ubm2,
            vertex_count: vc, index_count: ic, edge_vertex_count: evc, edge_index_count: eic,
            current_buffer_size: 0, current_index_buffer_size: 0,
            current_edge_buffer_size: 0, current_edge_index_buffer_size: 0,
            line_width: 2.0,
            uniforms: ubo, uniforms_ov: ubo_ov,
            eye: Vec3::new(0.0, 0.0, 5.0), target: Vec3::zero(),
            depth_format: dfmt, depth_image: dimg, depth_image_memory: dimg_mem,
            depth_image_view: dimg_view, current_image_index: 0,
            window_width: ext.width, window_height: ext.height,
            wireframe: false,
        }
    }

    fn find_qf(instance: &Instance, pd: vk::PhysicalDevice, surface: vk::SurfaceKHR, sfl: &surface::Instance) -> Option<u32> {
        let qfs = unsafe { instance.get_physical_device_queue_family_properties(pd) };
        for (i, f) in qfs.iter().enumerate() {
            if f.queue_flags.contains(vk::QueueFlags::GRAPHICS) && unsafe { sfl.get_physical_device_surface_support(pd, i as u32, surface) }.unwrap_or(false) { return Some(i as u32); }
        }
        None
    }

    fn create_sc(pd: vk::PhysicalDevice, sl: &swapchain::Device, surface: vk::SurfaceKHR, sfl: &surface::Instance, w: u32, h: u32) -> Option<(vk::SwapchainKHR, vk::Format, vk::Extent2D, Vec<vk::Image>)> {
        let caps = unsafe { sfl.get_physical_device_surface_capabilities(pd, surface) }.unwrap();
        // Если поверхность свернута (extent {0,0}) — не создаём свопчейн
        if caps.current_extent.width == 0 || caps.current_extent.height == 0 { return None; }
        let formats = unsafe { sfl.get_physical_device_surface_formats(pd, surface) }.unwrap();
        let fmt = formats.iter().find(|f| f.format == vk::Format::B8G8R8A8_SRGB || f.format == vk::Format::R8G8B8A8_SRGB).unwrap_or(&formats[0]);
        let ext = match caps.current_extent { vk::Extent2D { width: u32::MAX, .. } => vk::Extent2D { width: w, height: h }, e => e };
        let mut ic = caps.min_image_count + 1;
        if caps.max_image_count > 0 && ic > caps.max_image_count { ic = caps.max_image_count; }
        let sci = vk::SwapchainCreateInfoKHR::default().surface(surface).min_image_count(ic)
            .image_format(fmt.format).image_color_space(fmt.color_space).image_extent(ext)
            .image_array_layers(1).image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(vk::SharingMode::EXCLUSIVE).pre_transform(caps.current_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE).present_mode(vk::PresentModeKHR::FIFO).clipped(true);
        let sc = unsafe { sl.create_swapchain(&sci, None) }.unwrap();
        let imgs = unsafe { sl.get_swapchain_images(sc) }.unwrap();
        Some((sc, fmt.format, ext, imgs))
    }

    fn find_fmt(device: &Device, instance: &Instance, pd: vk::PhysicalDevice, candidates: &[vk::Format], tiling: vk::ImageTiling, features: vk::FormatFeatureFlags) -> Option<vk::Format> {
        for &f in candidates { let p = unsafe { instance.get_physical_device_format_properties(pd, f) }; let ok = match tiling { vk::ImageTiling::LINEAR => p.linear_tiling_features, _ => p.optimal_tiling_features }; if ok.contains(features) { return Some(f); } } None
    }

    fn create_depth(device: &Device, instance: &Instance, pd: vk::PhysicalDevice, w: u32, h: u32) -> (vk::Image, vk::DeviceMemory, vk::ImageView, vk::Format) {
        let df = Self::find_fmt(device, instance, pd, &[vk::Format::D32_SFLOAT, vk::Format::D24_UNORM_S8_UINT], vk::ImageTiling::OPTIMAL, vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT).unwrap();
        let ii = vk::ImageCreateInfo::default().image_type(vk::ImageType::TYPE_2D).format(df).extent(vk::Extent3D { width: w, height: h, depth: 1 }).mip_levels(1).array_layers(1).samples(vk::SampleCountFlags::TYPE_1).tiling(vk::ImageTiling::OPTIMAL).usage(vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT).sharing_mode(vk::SharingMode::EXCLUSIVE);
        let img = unsafe { device.create_image(&ii, None) }.unwrap();
        let mr = unsafe { device.get_image_memory_requirements(img) };
        let mt = Self::mt(instance, pd, mr.memory_type_bits, vk::MemoryPropertyFlags::DEVICE_LOCAL);
        let mem = unsafe { device.allocate_memory(&vk::MemoryAllocateInfo::default().allocation_size(mr.size).memory_type_index(mt), None) }.unwrap();
        unsafe { device.bind_image_memory(img, mem, 0) }.unwrap();
        let view = unsafe { device.create_image_view(&vk::ImageViewCreateInfo::default().image(img).view_type(vk::ImageViewType::TYPE_2D).format(df).subresource_range(vk::ImageSubresourceRange::default().aspect_mask(vk::ImageAspectFlags::DEPTH).base_mip_level(0).level_count(1).base_array_layer(0).layer_count(1)), None) }.unwrap();
        (img, mem, view, df)
    }

    fn create_rp(device: &Device, fmt: vk::Format, dfmt: vk::Format) -> vk::RenderPass {
        let ca = vk::AttachmentDescription::default().format(fmt).samples(vk::SampleCountFlags::TYPE_1).load_op(vk::AttachmentLoadOp::CLEAR).store_op(vk::AttachmentStoreOp::STORE).stencil_load_op(vk::AttachmentLoadOp::DONT_CARE).stencil_store_op(vk::AttachmentStoreOp::DONT_CARE).initial_layout(vk::ImageLayout::UNDEFINED).final_layout(vk::ImageLayout::PRESENT_SRC_KHR);
        let da = vk::AttachmentDescription::default().format(dfmt).samples(vk::SampleCountFlags::TYPE_1).load_op(vk::AttachmentLoadOp::CLEAR).store_op(vk::AttachmentStoreOp::DONT_CARE).stencil_load_op(vk::AttachmentLoadOp::DONT_CARE).stencil_store_op(vk::AttachmentStoreOp::DONT_CARE).initial_layout(vk::ImageLayout::UNDEFINED).final_layout(vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL);
        let cr = vk::AttachmentReference::default().attachment(0).layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL);
        let dr = vk::AttachmentReference::default().attachment(1).layout(vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL);
        let sp = vk::SubpassDescription::default().pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS).color_attachments(std::slice::from_ref(&cr)).depth_stencil_attachment(&dr);
        let dep = vk::SubpassDependency::default().src_subpass(vk::SUBPASS_EXTERNAL).dst_subpass(0).src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT | vk::PipelineStageFlags::LATE_FRAGMENT_TESTS).dst_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT | vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS).dst_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE | vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE);
        let atts = [ca, da];
        unsafe { device.create_render_pass(&vk::RenderPassCreateInfo::default().attachments(&atts).subpasses(std::slice::from_ref(&sp)).dependencies(std::slice::from_ref(&dep)), None) }.unwrap()
    }

    fn create_desc(device: &Device) -> (vk::DescriptorSetLayout, vk::DescriptorPool, vk::DescriptorSet, vk::DescriptorSet) {
        let b = vk::DescriptorSetLayoutBinding::default().binding(0).descriptor_type(vk::DescriptorType::UNIFORM_BUFFER).descriptor_count(1).stage_flags(vk::ShaderStageFlags::VERTEX);
        let l = unsafe { device.create_descriptor_set_layout(&vk::DescriptorSetLayoutCreateInfo::default().bindings(std::slice::from_ref(&b)), None) }.unwrap();
        let ps = [vk::DescriptorPoolSize::default().ty(vk::DescriptorType::UNIFORM_BUFFER).descriptor_count(2)];
        let p = unsafe { device.create_descriptor_pool(&vk::DescriptorPoolCreateInfo::default().max_sets(2).pool_sizes(&ps), None) }.unwrap();
        let layouts = [l, l];
        let sets = unsafe { device.allocate_descriptor_sets(&vk::DescriptorSetAllocateInfo::default().descriptor_pool(p).set_layouts(&layouts)) }.unwrap();
        (l, p, sets[0], sets[1])
    }

    fn mt(instance: &Instance, pd: vk::PhysicalDevice, bits: u32, props: vk::MemoryPropertyFlags) -> u32 {
        let mp = unsafe { instance.get_physical_device_memory_properties(pd) };
        for i in 0..mp.memory_type_count { if (bits & (1 << i)) != 0 && (mp.memory_types[i as usize].property_flags & props) == props { return i; } }
        panic!("no mem type");
    }

    fn create_vb(device: &Device, instance: &Instance, pd: vk::PhysicalDevice, data: &[f32]) -> (vk::Buffer, vk::DeviceMemory, u32) {
        let sz = (data.len() * 4) as u64;
        let b = unsafe { device.create_buffer(&vk::BufferCreateInfo::default().size(sz).usage(vk::BufferUsageFlags::VERTEX_BUFFER).sharing_mode(vk::SharingMode::EXCLUSIVE), None) }.unwrap();
        let mr = unsafe { device.get_buffer_memory_requirements(b) };
        let mt = Self::mt(instance, pd, mr.memory_type_bits, vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT);
        let m = unsafe { device.allocate_memory(&vk::MemoryAllocateInfo::default().allocation_size(mr.size).memory_type_index(mt), None) }.unwrap();
        unsafe { device.bind_buffer_memory(b, m, 0) }.unwrap();
        if !data.is_empty() {
            unsafe { let p = device.map_memory(m, 0, sz, vk::MemoryMapFlags::empty()).unwrap(); std::ptr::copy_nonoverlapping(data.as_ptr() as *const u8, p as *mut u8, sz as usize); device.unmap_memory(m); }
        }
        (b, m, (data.len() / 6) as u32)
    }

    fn create_ib(device: &Device, instance: &Instance, pd: vk::PhysicalDevice, data: &[u32]) -> (vk::Buffer, vk::DeviceMemory, u32) {
        let sz = (data.len() * 4) as u64;
        let b = unsafe { device.create_buffer(&vk::BufferCreateInfo::default().size(sz).usage(vk::BufferUsageFlags::INDEX_BUFFER).sharing_mode(vk::SharingMode::EXCLUSIVE), None) }.unwrap();
        let mr = unsafe { device.get_buffer_memory_requirements(b) };
        let mt = Self::mt(instance, pd, mr.memory_type_bits, vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT);
        let m = unsafe { device.allocate_memory(&vk::MemoryAllocateInfo::default().allocation_size(mr.size).memory_type_index(mt), None) }.unwrap();
        unsafe { device.bind_buffer_memory(b, m, 0) }.unwrap();
        if !data.is_empty() {
            unsafe { let p = device.map_memory(m, 0, sz, vk::MemoryMapFlags::empty()).unwrap(); std::ptr::copy_nonoverlapping(data.as_ptr() as *const u8, p as *mut u8, sz as usize); device.unmap_memory(m); }
        }
        (b, m, data.len() as u32)
    }

    fn create_ub(device: &Device, instance: &Instance, pd: vk::PhysicalDevice, ubo: &Uniforms) -> (vk::Buffer, vk::DeviceMemory) {
        let sz = std::mem::size_of::<Uniforms>() as u64;
        let b = unsafe { device.create_buffer(&vk::BufferCreateInfo::default().size(sz).usage(vk::BufferUsageFlags::UNIFORM_BUFFER).sharing_mode(vk::SharingMode::EXCLUSIVE), None) }.unwrap();
        let mr = unsafe { device.get_buffer_memory_requirements(b) };
        let mt = Self::mt(instance, pd, mr.memory_type_bits, vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT);
        let m = unsafe { device.allocate_memory(&vk::MemoryAllocateInfo::default().allocation_size(mr.size).memory_type_index(mt), None) }.unwrap();
        unsafe { device.bind_buffer_memory(b, m, 0) }.unwrap();
        unsafe { let p = device.map_memory(m, 0, sz, vk::MemoryMapFlags::empty()).unwrap(); std::ptr::copy_nonoverlapping(ubo as *const _ as *const u8, p as *mut u8, sz as usize); device.unmap_memory(m); }
        (b, m)
    }

    pub fn set_camera(&mut self, eye: Vec3, target: Vec3) { self.eye = eye; self.target = target; }

    pub fn set_wireframe(&mut self, on: bool) { self.wireframe = on; }

    fn update_ubo(&self, buf: vk::Buffer, mem: vk::DeviceMemory, ubo: &Uniforms) {
        let sz = std::mem::size_of::<Uniforms>() as u64;
        unsafe { let p = self.device.map_memory(mem, 0, sz, vk::MemoryMapFlags::empty()).unwrap(); std::ptr::copy_nonoverlapping(ubo as *const _ as *const u8, p as *mut u8, sz as usize); self.device.unmap_memory(mem); }
    }

    fn update_buf(&self, _buf: vk::Buffer, mem: vk::DeviceMemory, data: &[u8]) {
        if data.is_empty() { return; }
        let sz = data.len() as u64;
        unsafe {
            let p = self.device.map_memory(mem, 0, sz, vk::MemoryMapFlags::empty()).unwrap();
            std::ptr::copy_nonoverlapping(data.as_ptr(), p as *mut u8, data.len());
            self.device.unmap_memory(mem);
        }
    }

    fn rebuild_bufs(&mut self, verts: &[f32], idxs: &[u32]) {
        let vsz = (verts.len() * 4) as u64; let isz = (idxs.len() * 4) as u64;
        if vsz != self.current_buffer_size {
            unsafe { self.device.destroy_buffer(self.vertex_buffer, None); self.device.free_memory(self.vertex_buffer_memory, None); }
            let (vb, vm, vc) = Self::create_vb(&self.device, &self.instance, self.physical_device, verts);
            self.vertex_buffer = vb; self.vertex_buffer_memory = vm; self.vertex_count = vc; self.current_buffer_size = vsz;
        } else {
            // Size unchanged — just upload new data into existing buffer
            let vert_bytes = unsafe { std::slice::from_raw_parts(verts.as_ptr() as *const u8, verts.len() * 4) };
            self.update_buf(self.vertex_buffer, self.vertex_buffer_memory, vert_bytes);
        }
        if isz != self.current_index_buffer_size {
            unsafe { self.device.destroy_buffer(self.index_buffer, None); self.device.free_memory(self.index_buffer_memory, None); }
            let (ib, im, ic) = Self::create_ib(&self.device, &self.instance, self.physical_device, idxs);
            self.index_buffer = ib; self.index_buffer_memory = im; self.index_count = ic; self.current_index_buffer_size = isz;
        } else {
            let idx_bytes = unsafe { std::slice::from_raw_parts(idxs.as_ptr() as *const u8, idxs.len() * 4) };
            self.update_buf(self.index_buffer, self.index_buffer_memory, idx_bytes);
        }
    }

    fn rebuild_edge_bufs(&mut self, verts: &[f32], idxs: &[u32]) {
        if verts.is_empty() || idxs.is_empty() { return; }
        let vsz = (verts.len() * 4) as u64; let isz = (idxs.len() * 4) as u64;
        if vsz != self.current_edge_buffer_size {
            if self.current_edge_buffer_size > 0 { unsafe { self.device.destroy_buffer(self.edge_vertex_buffer, None); self.device.free_memory(self.edge_vertex_buffer_memory, None); } }
            let (vb, vm, vc) = Self::create_vb(&self.device, &self.instance, self.physical_device, verts);
            self.edge_vertex_buffer = vb; self.edge_vertex_buffer_memory = vm; self.edge_vertex_count = vc; self.current_edge_buffer_size = vsz;
        } else {
            let vert_bytes = unsafe { std::slice::from_raw_parts(verts.as_ptr() as *const u8, verts.len() * 4) };
            self.update_buf(self.edge_vertex_buffer, self.edge_vertex_buffer_memory, vert_bytes);
        }
        if isz != self.current_edge_index_buffer_size {
            if self.current_edge_index_buffer_size > 0 { unsafe { self.device.destroy_buffer(self.edge_index_buffer, None); self.device.free_memory(self.edge_index_buffer_memory, None); } }
            let (ib, im, ic) = Self::create_ib(&self.device, &self.instance, self.physical_device, idxs);
            self.edge_index_buffer = ib; self.edge_index_buffer_memory = im; self.edge_index_count = ic; self.current_edge_index_buffer_size = isz;
        } else {
            let idx_bytes = unsafe { std::slice::from_raw_parts(idxs.as_ptr() as *const u8, idxs.len() * 4) };
            self.update_buf(self.edge_index_buffer, self.edge_index_buffer_memory, idx_bytes);
        }
    }

    pub fn render_scene(&mut self, scene: &Scene, window_w: u32, window_h: u32, hover: u8) {
        // Окно свернуто (размер 0) — не рендерим, hover игнорируем
        if window_w == 0 || window_h == 0 { return; }
        // Поверхность не рендерится (свёрнуто/скрыто) — драйвер отдаёт extent {0,0},
        // даже если GLFW ещё хранит старый размер окна
        unsafe {
            if let Ok(caps) = self.surface_loader.get_physical_device_surface_capabilities(self.physical_device, self.surface) {
                if caps.current_extent.width == 0 || caps.current_extent.height == 0 {
                    return;
                }
            }
        }
        unsafe { self.device.wait_for_fences(std::slice::from_ref(&self.fence), true, u64::MAX).unwrap(); self.device.reset_fences(std::slice::from_ref(&self.fence)).unwrap(); }
        // Если размер окна изменился — пересоздаём свопчейн
        if window_w != self.window_width || window_h != self.window_height {
            self.rebuild_swapchain(window_w, window_h);
        }
        let image_index = match unsafe { self.swapchain_loader.acquire_next_image(self.swapchain, u64::MAX, self.image_available_semaphore, self.fence) } {
            Ok((idx, _)) => idx, Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => { self.rebuild_swapchain(window_w, window_h); return; } Err(_) => return,
        };
        unsafe { self.device.wait_for_fences(std::slice::from_ref(&self.fence), true, u64::MAX).unwrap(); self.device.reset_fences(std::slice::from_ref(&self.fence)).unwrap(); }

        let mut all_v = Vec::new();
        let mut all_i = Vec::new();
        let mut edge_v = Vec::new();
        let mut edge_i = Vec::new();
        for obj in scene.get_objects() {
            let m = &obj.mesh; let vs = &m.vertices; let is = &m.indices; let c = obj.color.as_array();
            let base = all_v.len() / 6;
            for ch in vs.chunks(3) { let tv = obj.transform.model.transform(&Vec3::new(ch[0], ch[1], ch[2])); all_v.extend_from_slice(&[tv.x, tv.y, tv.z, c[0], c[1], c[2]]); }
            for &ix in is { all_i.push(ix + base as u32); }
            // Рёбра: для каждого треугольника добавляем три ребра (дедупликация)
            if is.len() >= 3 {
                use std::collections::HashSet;
                let mut seen: HashSet<(u32, u32)> = HashSet::new();
                let _ebase = edge_v.len() / 6;
                for tri in is.chunks(3) {
                    if tri.len() < 3 { break; }
                    let edges = [(tri[0], tri[1]), (tri[1], tri[2]), (tri[2], tri[0])];
                    for &(a, b) in &edges {
                        let key = if a < b { (a, b) } else { (b, a) };
                        if seen.insert(key) {
                            // Трансформируем оба конца ребра в мировые координаты
                            let va = &vs[(a as usize) * 3..];
                            let vb = &vs[(b as usize) * 3..];
                            let pa = obj.transform.model.transform(&Vec3::new(va[0], va[1], va[2]));
                            let pb = obj.transform.model.transform(&Vec3::new(vb[0], vb[1], vb[2]));
                            edge_v.extend_from_slice(&[pa.x, pa.y, pa.z, 0.0, 0.0, 0.0]);
                            edge_v.extend_from_slice(&[pb.x, pb.y, pb.z, 0.0, 0.0, 0.0]);
                            let ei = edge_v.len() / 6 - 2;
                            edge_i.push(ei as u32);
                            edge_i.push(ei as u32 + 1);
                        }
                    }
                }
            }
        }
        let scene_ic = all_i.len() as u32;
        let edge_ic = edge_i.len() as u32;

        // Overlay: панель 40px с зазором 10px от краёв окна, скругление 13px;
        // кнопки 28x28 со скруглением 13px, внизу панели (красная ниже, жёлтая выше),
        // зазор 5px между кнопками
        // NDC: y=-1 = верх framebuffer, y=1 = низ; диапазон = 2.0
        let w = self.swapchain_extent.width.max(1) as f32;
        let h = self.swapchain_extent.height.max(1) as f32;
        let cx13 = 30.0 / w;        // радиус панели 15px по X
        let cy13 = 30.0 / h;        // радиус панели 15px по Y
        let btn_rx = 20.0 / w;      // радиус кнопок 10px по X
        let btn_ry = 20.0 / h;      // радиус кнопок 10px по Y
        let gap_r = 20.0 / w;       // 10px зазор справа (панель не прилегает к краю)
        let gap_v = 20.0 / h;       // 10px зазор сверху и снизу
        let pw_ = 80.0 / w;         // ширина панели 40px
        let panel_right = 1.0 - gap_r;
        let panel_left = panel_right - pw_;
        // y_world: +1 = ВЕРХ экрана, -1 = НИЗ (после Y-flip в ортопроекции)
        let panel_top = 1.0 - gap_v;
        let panel_bottom = -1.0 + gap_v;
        // Кнопки внутри панели
        let btn_sz_x = 56.0 / w;    // 28px
        let btn_sz_y = 56.0 / h;
        let btn_left = panel_left + 12.0 / w; // 6px от левого края панели
        let btn_right = btn_left + btn_sz_x;
        let btn_pad_b = 10.0 / h;   // 5px от низа панели
        let gap_b = 10.0 / h;       // зазор 5px между кнопками
        // Красная внизу: низ кнопки на 5px выше низа панели
        let red_bottom = panel_bottom + btn_pad_b;
        let red_top = red_bottom + btn_sz_y;
        // Жёлтая выше с зазором 5px
        let yel_bottom = red_top + gap_b;
        let _yel_top = yel_bottom + btn_sz_y;

        // hover цвета подсветки
        let (y_r, y_g, y_b) = if hover == 1 { (1.0, 0.95, 0.4) } else { (1.0, 0.882, 0.0) }; // #FFE100
        let (r_r, r_g, r_b) = if hover == 2 { (1.0, 0.3, 0.3) } else { (1.0, 0.0, 0.0) }; // #FF0000

        let zd = 0.0;

        // helper: rounded rect — настоящие дуги закругления
        // (left, bottom, right, top) в world-координатах
        // y_world +1 = верх экрана, -1 = низ
        // Каждая дуга даёт `segs` точек (без последней — она = первой следующей),
        // периметр замкнут = 4*segs уникальных вершин; триангуляция веером от центра
        macro_rules! rounded_rect {
            ($v:expr, $i:expr, $base:expr, $left:expr, $bottom:expr, $right:expr, $top:expr, $rx:expr, $ry:expr, $segs:expr, $r:expr, $g:expr, $b:expr) => {{
                let b = $base as u32;
                let segs = $segs;
                let step = (std::f32::consts::PI / 2.0) / segs as f32;
                let arcs: [(f32, f32, f32); 4] = [
                    ($left + $rx, $bottom + $ry, std::f32::consts::PI), // BL: PI → 3PI/2
                    ($right - $rx, $bottom + $ry, 3.0 * std::f32::consts::PI / 2.0), // BR: 3PI/2 → 2PI
                    ($right - $rx, $top - $ry, 0.0), // TR: 0 → PI/2
                    ($left + $rx, $top - $ry, std::f32::consts::PI / 2.0), // TL: PI/2 → PI
                ];
                for &(ccx, ccy, start) in &arcs {
                    for s in 0..segs {
                        let a = start + step * s as f32;
                        $v.extend_from_slice(&[ccx + $rx * a.cos(), ccy + $ry * a.sin(), zd, $r, $g, $b]);
                    }
                }
                // 4*segs вершин периметра + 1 центр = 4*segs+1
                let total_perim = 4u32 * (segs as u32);
                let ctr = b + total_perim;
                $v.extend_from_slice(&[($left + $right) * 0.5, ($bottom + $top) * 0.5, zd, $r, $g, $b]);
                for k in 0..total_perim {
                    let n = if k + 1 < total_perim { k + 1 } else { 0 };
                    $i.extend_from_slice(&[ctr, b + k, b + n]);
                }
            }};
        }

        // helper: rect (two triangles) — для иконок
        macro_rules! rect {
            ($v:expr, $i:expr, $base:expr, $x1:expr, $y1:expr, $x2:expr, $y2:expr, $r:expr, $g:expr, $b:expr) => {{
                let b = $base;
                $v.extend_from_slice(&[
                    $x1, $y1, zd, $r,$g,$b,  $x2, $y1, zd, $r,$g,$b,
                    $x2, $y2, zd, $r,$g,$b,  $x1, $y2, zd, $r,$g,$b,
                ]);
                $i.extend_from_slice(&[b,b+1,b+2, b,b+2,b+3]);
            }};
        }

        let segs = 20; // сегментов на угол — достаточно для плавности
        let verts_per = (4 * segs + 1) as u32; // 4 дуги*segs периметра + 1 центр
        let ov_base = (all_v.len() / 6) as u32;
        // Панель: скруглённый прямоугольник, серая #949494
        rounded_rect!(all_v, all_i, ov_base, panel_left, panel_bottom, panel_right, panel_top, cx13, cy13, segs, 0.580, 0.580, 0.580);
        // Red (нижняя кнопка, радиус 10px) — закрыть
        let b2 = ov_base + verts_per;
        rounded_rect!(all_v, all_i, b2, btn_left, red_bottom, btn_right, red_top, btn_rx, btn_ry, segs, r_r, r_g, r_b);
        // Иконка крестика (только при hover == 2)
        if hover == 2 {
            let cx = (btn_left + btn_right) * 0.5;
            let cy = (red_bottom + red_top) * 0.5;
            let r2 = 0.70710678;
            let rad = 8.0 / w;
            let th = 2.0 / w;
            let d = rad * r2;
            let t = th * r2;
            let b2i = b2 + verts_per;
            all_v.extend_from_slice(&[
                cx+d+t, cy+d-t, zd, 0.15,0.15,0.15,
                cx+d-t, cy+d+t, zd, 0.15,0.15,0.15,
                cx-d-t, cy-d+t, zd, 0.15,0.15,0.15,
                cx-d+t, cy-d-t, zd, 0.15,0.15,0.15,
            ]);
            all_i.extend_from_slice(&[b2i,b2i+1,b2i+2, b2i,b2i+2,b2i+3]);
            let b2i2 = b2i + 4;
            all_v.extend_from_slice(&[
                cx+d+t, cy-d+t, zd, 0.15,0.15,0.15,
                cx+d-t, cy-d-t, zd, 0.15,0.15,0.15,
                cx-d-t, cy+d+t, zd, 0.15,0.15,0.15,
                cx-d+t, cy+d-t, zd, 0.15,0.15,0.15,
            ]);
            all_i.extend_from_slice(&[b2i2,b2i2+1,b2i2+2, b2i2,b2i2+2,b2i2+3]);
        }
        // Yellow (выше красной, зазор 5px, радиус 10px) — свернуть
        let b3 = if hover == 2 { b2 + verts_per + 8 } else { b2 + verts_per };
        rounded_rect!(all_v, all_i, b3, btn_left, yel_bottom, btn_right, yel_bottom+btn_sz_y, btn_rx, btn_ry, segs, y_r, y_g, y_b);
        // Иконка минуса (только при hover == 1)
        if hover == 1 {
            let cx = (btn_left + btn_right) * 0.5;
            let cy = (yel_bottom + yel_bottom+btn_sz_y) * 0.5;
            let hw = 10.0 / w;
            let ht = 3.0 / w;
            let b3i = b3 + verts_per;
            rect!(all_v, all_i, b3i, cx-hw, cy-ht, cx+hw, cy+ht, 0.15, 0.15, 0.15);
        }

        let aspect = self.swapchain_extent.width as f32 / self.swapchain_extent.height as f32;
        self.uniforms.projection = Mat4::perspective(45.0_f32.to_radians(), aspect, 0.1, 100.0);
        self.uniforms.view = Mat4::look_at(self.eye, self.target, Vec3::new(0.0, 1.0, 0.0));
        self.uniforms.model = Mat4::identity();
        self.update_ubo(self.uniform_buffer, self.uniform_buffer_memory, &self.uniforms);

        self.uniforms_ov.projection = Mat4::orthographic(-1.0, 1.0, -1.0, 1.0, 0.0, 10.0);
        self.uniforms_ov.view = Mat4::identity();
        self.uniforms_ov.model = Mat4::identity();
        self.update_ubo(self.uniform_buffer_ov, self.uniform_buffer_ov_memory, &self.uniforms_ov);

        self.rebuild_bufs(&all_v, &all_i);
        self.rebuild_edge_bufs(&edge_v, &edge_i);

        let cmd = self.command_buffers[image_index as usize];
        unsafe {
            self.device.reset_command_buffer(cmd, vk::CommandBufferResetFlags::empty()).unwrap();
            self.device.begin_command_buffer(cmd, &vk::CommandBufferBeginInfo::default()).unwrap();
            let cc = vk::ClearColorValue { float32: [0.63, 0.91, 1.0, 1.0] };
            let cd = vk::ClearDepthStencilValue { depth: 1.0, stencil: 0 };
            let cv = [vk::ClearValue { color: cc }, vk::ClearValue { depth_stencil: cd }];
            let rpbi = vk::RenderPassBeginInfo::default().render_pass(self.render_pass)
                .framebuffer(self.framebuffers[image_index as usize])
                .render_area(vk::Rect2D::default().offset(vk::Offset2D{x:0,y:0}).extent(self.swapchain_extent))
                .clear_values(&cv);
            self.device.cmd_begin_render_pass(cmd, &rpbi, vk::SubpassContents::INLINE);
            self.device.cmd_bind_pipeline(cmd, vk::PipelineBindPoint::GRAPHICS, self.pipeline.handle);
            // Динамический viewport/scissor на текущий размер
            let vp = vk::Viewport::default()
                .x(0.0).y(0.0)
                .width(self.swapchain_extent.width as f32)
                .height(self.swapchain_extent.height as f32)
                .min_depth(0.0).max_depth(1.0);
            let sc = vk::Rect2D::default()
                .offset(vk::Offset2D { x: 0, y: 0 })
                .extent(self.swapchain_extent);
            self.device.cmd_set_viewport(cmd, 0, &[vp]);
            self.device.cmd_set_scissor(cmd, 0, &[sc]);
            self.device.cmd_bind_vertex_buffers(cmd, 0, &[self.vertex_buffer], &[0]);
            self.device.cmd_bind_index_buffer(cmd, self.index_buffer, 0, vk::IndexType::UINT32);

            // Draw scene (perspective) — draws on top
            self.device.cmd_bind_descriptor_sets(cmd, vk::PipelineBindPoint::GRAPHICS, self.pipeline_layout, 0, std::slice::from_ref(&self.descriptor_set_scene), &[]);
            if scene_ic > 0 { self.device.cmd_draw_indexed(cmd, scene_ic, 1, 0, 0, 0); }

            // Draw edges (2px black lines on crease edges) — только если wireframe включён в сцене
            if self.wireframe && edge_ic > 0 {
                self.device.cmd_bind_pipeline(cmd, vk::PipelineBindPoint::GRAPHICS, self.pipeline.line_handle);
                self.device.cmd_bind_descriptor_sets(cmd, vk::PipelineBindPoint::GRAPHICS, self.pipeline_layout, 0, std::slice::from_ref(&self.descriptor_set_scene), &[]);
                self.device.cmd_bind_vertex_buffers(cmd, 0, &[self.edge_vertex_buffer], &[0]);
                self.device.cmd_bind_index_buffer(cmd, self.edge_index_buffer, 0, vk::IndexType::UINT32);
                self.device.cmd_draw_indexed(cmd, edge_ic, 1, 0, 0, 0);
                // Возвращаем обычный пайплайн для оверлея
                self.device.cmd_bind_pipeline(cmd, vk::PipelineBindPoint::GRAPHICS, self.pipeline.handle);
                self.device.cmd_bind_vertex_buffers(cmd, 0, &[self.vertex_buffer], &[0]);
                self.device.cmd_bind_index_buffer(cmd, self.index_buffer, 0, vk::IndexType::UINT32);
            }

            // Draw overlay LAST (on top, z=0 == cleared depth, LESS_OR_EQUAL passes)
            self.device.cmd_bind_descriptor_sets(cmd, vk::PipelineBindPoint::GRAPHICS, self.pipeline_layout, 0, std::slice::from_ref(&self.descriptor_set_ov), &[]);
            let ov_ic = all_i.len() as u32 - scene_ic;
            if ov_ic > 0 { self.device.cmd_draw_indexed(cmd, ov_ic, 1, scene_ic, 0, 0); }

            self.device.cmd_end_render_pass(cmd);
            self.device.end_command_buffer(cmd).unwrap();
        }

        let si = vk::SubmitInfo::default().wait_semaphores(std::slice::from_ref(&self.image_available_semaphore))
            .wait_dst_stage_mask(std::slice::from_ref(&vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT))
            .command_buffers(std::slice::from_ref(&cmd)).signal_semaphores(std::slice::from_ref(&self.render_finished_semaphore));
        unsafe { self.device.queue_submit(self.queue, std::slice::from_ref(&si), self.fence).unwrap(); }
        let pi = vk::PresentInfoKHR::default().wait_semaphores(std::slice::from_ref(&self.render_finished_semaphore))
            .swapchains(std::slice::from_ref(&self.swapchain)).image_indices(std::slice::from_ref(&image_index));
        unsafe {
            match self.swapchain_loader.queue_present(self.queue, &pi) {
                Ok(true) | Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => {
                    self.rebuild_swapchain(window_w, window_h);
                }
                _ => {}
            }
        }
    }

    fn rebuild_swapchain(&mut self, w: u32, h: u32) {
        if w == 0 || h == 0 { return; }
        // Сначала создаём новый свопчейн (поверхность может быть недоступна —
        // тогда ничего не трогаем, старый продолжит жить)
        let (sc, fmt, ext, imgs) = match Self::create_sc(self.physical_device, &self.swapchain_loader, self.surface, &self.surface_loader, w, h) {
            Some(v) => v, None => return,
        };
        unsafe { self.device.queue_wait_idle(self.queue).unwrap(); }
        unsafe { for &fb in &self.framebuffers { self.device.destroy_framebuffer(fb, None); } for &view in &self.swapchain_image_views { self.device.destroy_image_view(view, None); } self.device.destroy_image_view(self.depth_image_view, None); self.device.destroy_image(self.depth_image, None); self.device.free_memory(self.depth_image_memory, None); self.swapchain_loader.destroy_swapchain(self.swapchain, None); }
        self.window_width = w.max(1); self.window_height = h.max(1);
        self.swapchain = sc; self.swapchain_format = fmt; self.swapchain_extent = ext; self.swapchain_images = imgs;
        self.swapchain_image_views = self.swapchain_images.iter().map(|&img| unsafe { self.device.create_image_view(&vk::ImageViewCreateInfo::default().image(img).view_type(vk::ImageViewType::TYPE_2D).format(fmt).subresource_range(vk::ImageSubresourceRange::default().aspect_mask(vk::ImageAspectFlags::COLOR).base_mip_level(0).level_count(1).base_array_layer(0).layer_count(1)), None) }.unwrap()).collect();
        let (dimg, dimg_mem, dimg_view, dfmt) = Self::create_depth(&self.device, &self.instance, self.physical_device, w, h);
        self.depth_image = dimg; self.depth_image_memory = dimg_mem; self.depth_image_view = dimg_view; self.depth_format = dfmt;
        self.framebuffers = self.swapchain_image_views.iter().map(|&view| { let att = [view, self.depth_image_view]; unsafe { self.device.create_framebuffer(&vk::FramebufferCreateInfo::default().render_pass(self.render_pass).attachments(&att).width(ext.width).height(ext.height).layers(1), None) }.unwrap() }).collect();
    }

    pub fn cleanup(&mut self) {
        unsafe { self.device.queue_wait_idle(self.queue).unwrap(); }
        unsafe {
            self.pipeline.cleanup(&self.device);
            self.device.destroy_pipeline_layout(self.pipeline_layout, None);
            self.device.destroy_descriptor_pool(self.descriptor_pool, None);
            self.device.destroy_descriptor_set_layout(self.descriptor_set_layout, None);
            self.device.destroy_buffer(self.vertex_buffer, None); self.device.free_memory(self.vertex_buffer_memory, None);
            self.device.destroy_buffer(self.index_buffer, None); self.device.free_memory(self.index_buffer_memory, None);
            self.device.destroy_buffer(self.edge_vertex_buffer, None); self.device.free_memory(self.edge_vertex_buffer_memory, None);
            self.device.destroy_buffer(self.edge_index_buffer, None); self.device.free_memory(self.edge_index_buffer_memory, None);
            self.device.destroy_buffer(self.uniform_buffer, None); self.device.free_memory(self.uniform_buffer_memory, None);
            self.device.destroy_buffer(self.uniform_buffer_ov, None); self.device.free_memory(self.uniform_buffer_ov_memory, None);
            self.device.destroy_image_view(self.depth_image_view, None); self.device.destroy_image(self.depth_image, None); self.device.free_memory(self.depth_image_memory, None);
            self.device.destroy_fence(self.fence, None); self.device.destroy_semaphore(self.image_available_semaphore, None); self.device.destroy_semaphore(self.render_finished_semaphore, None);
            self.device.destroy_command_pool(self.command_pool, None); self.device.destroy_render_pass(self.render_pass, None);
            for &fb in &self.framebuffers { self.device.destroy_framebuffer(fb, None); }
            for &view in &self.swapchain_image_views { self.device.destroy_image_view(view, None); }
            self.swapchain_loader.destroy_swapchain(self.swapchain, None);
            self.device.destroy_device(None); self.surface_loader.destroy_surface(self.surface, None); self.instance.destroy_instance(None);
        }
    }
}