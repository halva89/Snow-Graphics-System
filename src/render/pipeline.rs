use ash::vk;
use std::ffi::CString;
use std::fs;

pub struct Pipeline {
    pub layout: vk::PipelineLayout,
    pub handle: vk::Pipeline,
    pub line_handle: vk::Pipeline,
}

impl Pipeline {
    pub fn new(device: &ash::Device, render_pass: vk::RenderPass, swapchain_extent: vk::Extent2D) -> Self {
        let layout = unsafe { device.create_pipeline_layout(&vk::PipelineLayoutCreateInfo::default(), None) }
            .expect("Failed to create pipeline layout");
        Self::new_with_layout(device, render_pass, swapchain_extent, layout, 1.0)
    }

    pub fn new_with_layout(
        device: &ash::Device, 
        render_pass: vk::RenderPass, 
        _swapchain_extent: vk::Extent2D,
        layout: vk::PipelineLayout,
        line_width: f32,
    ) -> Self {
        println!("[Pipeline] Creating pipeline... (line_width = {})", line_width);
        
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

        // Dynamic viewport — ставим заглушку, реальный размер задаётся каждый кадр
        let viewport = vk::Viewport::default()
            .x(0.0).y(0.0).width(1.0).height(1.0).min_depth(0.0).max_depth(1.0);
        let scissor = vk::Rect2D::default()
            .offset(vk::Offset2D { x: 0, y: 0 })
            .extent(vk::Extent2D { width: 1, height: 1 });

        let viewport_state = vk::PipelineViewportStateCreateInfo::default()
            .viewports(std::slice::from_ref(&viewport))
            .scissors(std::slice::from_ref(&scissor));

        let dynamic_states = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
        let dynamic_state = vk::PipelineDynamicStateCreateInfo::default()
            .dynamic_states(&dynamic_states);

        let rasterizer = vk::PipelineRasterizationStateCreateInfo::default()
            .polygon_mode(vk::PolygonMode::FILL)
            .line_width(1.0)
            .cull_mode(vk::CullModeFlags::NONE)
            .front_face(vk::FrontFace::COUNTER_CLOCKWISE);

        // Линейный пайплайн: чёрные рёбра граней (crease edges)
        let line_assembly = vk::PipelineInputAssemblyStateCreateInfo::default()
            .topology(vk::PrimitiveTopology::LINE_LIST);
        let line_rasterizer = vk::PipelineRasterizationStateCreateInfo::default()
            .polygon_mode(vk::PolygonMode::FILL)
            .line_width(line_width)
            .cull_mode(vk::CullModeFlags::NONE)
            .front_face(vk::FrontFace::COUNTER_CLOCKWISE)
            // Сдвигаем линии чуть к камере, чтобы не было z-fighting с гранями
            .depth_bias_enable(true)
            .depth_bias_constant_factor(-2.0)
            .depth_bias_slope_factor(-1.0);

        let multisample = vk::PipelineMultisampleStateCreateInfo::default()
            .rasterization_samples(vk::SampleCountFlags::TYPE_1);

        let depth_stencil = vk::PipelineDepthStencilStateCreateInfo::default()
            .depth_test_enable(true)
            .depth_write_enable(true)
            .depth_compare_op(vk::CompareOp::LESS_OR_EQUAL);

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
            .dynamic_state(&dynamic_state)
            .rasterization_state(&rasterizer)
            .multisample_state(&multisample)
            .depth_stencil_state(&depth_stencil)
            .color_blend_state(&color_blend)
            .layout(layout)
            .render_pass(render_pass)
            .subpass(0);

        let line_info = vk::GraphicsPipelineCreateInfo::default()
            .stages(&stages)
            .vertex_input_state(&vertex_input)
            .input_assembly_state(&line_assembly)
            .viewport_state(&viewport_state)
            .dynamic_state(&dynamic_state)
            .rasterization_state(&line_rasterizer)
            .multisample_state(&multisample)
            .depth_stencil_state(&depth_stencil)
            .color_blend_state(&color_blend)
            .layout(layout)
            .render_pass(render_pass)
            .subpass(0);

        let pipelines = unsafe {
            device.create_graphics_pipelines(vk::PipelineCache::null(), std::slice::from_ref(&pipeline_info), None)
        };
        let line_pipelines = unsafe {
            device.create_graphics_pipelines(vk::PipelineCache::null(), std::slice::from_ref(&line_info), None)
        };

        unsafe {
            device.destroy_shader_module(vert_module, None);
            device.destroy_shader_module(frag_module, None);
        }

        match (pipelines, line_pipelines) {
            (Ok(p), Ok(lp)) => {
                println!("[Pipeline] Pipelines created successfully! (fill + line)");
                Self { layout, handle: p[0], line_handle: lp[0] }
            }
            (Err(e), _) | (_, Err(e)) => {
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
            device.destroy_pipeline(self.line_handle, None);
        }
    }
}