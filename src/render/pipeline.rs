use ash::vk;
use std::ffi::CString;
use std::fs;

pub struct Pipeline {
    pub layout: vk::PipelineLayout,
    pub handle: vk::Pipeline,
}

impl Pipeline {
    pub fn new(device: &ash::Device, render_pass: vk::RenderPass, swapchain_extent: vk::Extent2D) -> Self {
        println!("[Pipeline] Creating pipeline...");
        
        let vert_path = "shaders/vert.spv";
        let frag_path = "shaders/frag.spv";
        
        let vert_code = Self::load_shader(vert_path);
        let frag_code = Self::load_shader(frag_path);

        let vert_module = Self::create_shader_module(device, &vert_code);
        let frag_module = Self::create_shader_module(device, &frag_code);

        let name = CString::new("main").unwrap();

        let vert_stage = vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlags::VERTEX)
            .module(vert_module)
            .name(&name);

        let frag_stage = vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlags::FRAGMENT)
            .module(frag_module)
            .name(&name);

        let stages = [vert_stage, frag_stage];

        let layout_info = vk::PipelineLayoutCreateInfo::default();
        let layout = unsafe { device.create_pipeline_layout(&layout_info, None) }.unwrap();

        let binding_desc = vk::VertexInputBindingDescription::default()
            .binding(0)
            .stride(6 * std::mem::size_of::<f32>() as u32)
            .input_rate(vk::VertexInputRate::VERTEX);

        let attr_descs = [
            vk::VertexInputAttributeDescription::default()
                .location(0)
                .binding(0)
                .format(vk::Format::R32G32B32_SFLOAT)
                .offset(0),
            vk::VertexInputAttributeDescription::default()
                .location(1)
                .binding(0)
                .format(vk::Format::R32G32B32_SFLOAT)
                .offset(3 * std::mem::size_of::<f32>() as u32),
        ];

        let vertex_input = vk::PipelineVertexInputStateCreateInfo::default()
            .vertex_binding_descriptions(std::slice::from_ref(&binding_desc))
            .vertex_attribute_descriptions(&attr_descs);

        let input_assembly = vk::PipelineInputAssemblyStateCreateInfo::default()
            .topology(vk::PrimitiveTopology::TRIANGLE_LIST);

        // Вьюпорт с правильным соотношением сторон
        let aspect = swapchain_extent.width as f32 / swapchain_extent.height as f32;
        let viewport = vk::Viewport::default()
            .x(0.0)
            .y(0.0)
            .width(swapchain_extent.width as f32)
            .height(swapchain_extent.height as f32)
            .min_depth(0.0)
            .max_depth(1.0);

        let scissor = vk::Rect2D::default()
            .offset(vk::Offset2D { x: 0, y: 0 })
            .extent(swapchain_extent);

        let viewport_state = vk::PipelineViewportStateCreateInfo::default()
            .viewports(std::slice::from_ref(&viewport))
            .scissors(std::slice::from_ref(&scissor));

        let rasterizer = vk::PipelineRasterizationStateCreateInfo::default()
            .polygon_mode(vk::PolygonMode::FILL)
            .line_width(1.0)
            .cull_mode(vk::CullModeFlags::NONE)
            .front_face(vk::FrontFace::COUNTER_CLOCKWISE);

        let multisample = vk::PipelineMultisampleStateCreateInfo::default()
            .rasterization_samples(vk::SampleCountFlags::TYPE_1);

        let color_blend_attachment = vk::PipelineColorBlendAttachmentState::default()
            .color_write_mask(vk::ColorComponentFlags::RGBA)
            .blend_enable(false);

        let color_blend = vk::PipelineColorBlendStateCreateInfo::default()
            .attachments(std::slice::from_ref(&color_blend_attachment));

        let pipeline_info = vk::GraphicsPipelineCreateInfo::default()
            .stages(&stages)
            .vertex_input_state(&vertex_input)
            .input_assembly_state(&input_assembly)
            .viewport_state(&viewport_state)
            .rasterization_state(&rasterizer)
            .multisample_state(&multisample)
            .color_blend_state(&color_blend)
            .layout(layout)
            .render_pass(render_pass)
            .subpass(0);

        let pipeline = unsafe {
            device.create_graphics_pipelines(vk::PipelineCache::null(), std::slice::from_ref(&pipeline_info), None)
        };

        unsafe {
            device.destroy_shader_module(vert_module, None);
            device.destroy_shader_module(frag_module, None);
        }

        match pipeline {
            Ok(pipelines) => {
                println!("[Pipeline] Pipeline created successfully!");
                Self { layout, handle: pipelines[0] }
            }
            Err(e) => {
                panic!("Failed to create pipeline: {:?}", e);
            }
        }
    }

    fn load_shader(path: &str) -> Vec<u32> {
        let bytes = fs::read(path).expect(&format!("Failed to read shader file: {}", path));
        
        let code = bytes.chunks_exact(4)
            .map(|c| u32::from_le_bytes(c.try_into().unwrap()))
            .collect::<Vec<_>>();
        
        code
    }

    fn create_shader_module(device: &ash::Device, code: &[u32]) -> vk::ShaderModule {
        unsafe {
            device.create_shader_module(
                &vk::ShaderModuleCreateInfo::default().code(code),
                None,
            )
            .expect("Failed to create shader module")
        }
    }

    pub fn cleanup(&self, device: &ash::Device) {
        unsafe {
            device.destroy_pipeline(self.handle, None);
            device.destroy_pipeline_layout(self.layout, None);
        }
    }
}