use std::sync::Arc;
use std::error::Error;
use vulkano::VulkanLibrary;
use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::device::{Device, Queue, DeviceCreateInfo, QueueCreateInfo, DeviceExtensions, QueueFlags};
use vulkano::swapchain::{Surface, Swapchain, SwapchainCreateInfo, SwapchainPresentInfo};
use vulkano::buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage, Subbuffer};
use vulkano::image::{ImageUsage, view::ImageView};
use vulkano::memory::allocator::{StandardMemoryAllocator, AllocationCreateInfo, MemoryTypeFilter};
use vulkano::pipeline::graphics::vertex_input::{Vertex, VertexDefinition};
use vulkano::pipeline::graphics::viewport::{Viewport, ViewportState};
use vulkano::pipeline::graphics::GraphicsPipelineCreateInfo;
use vulkano::pipeline::layout::PipelineLayoutCreateInfo;
use vulkano::pipeline::{GraphicsPipeline, PipelineLayout, PipelineShaderStageCreateInfo, Pipeline};
use vulkano::pipeline::graphics::input_assembly::InputAssemblyState;
use vulkano::pipeline::graphics::rasterization::{RasterizationState, FrontFace};
use vulkano::pipeline::graphics::multisample::MultisampleState;
use vulkano::pipeline::graphics::color_blend::{ColorBlendState, ColorBlendAttachmentState};
use vulkano::render_pass::{Framebuffer, FramebufferCreateInfo, Subpass};
use vulkano::command_buffer::allocator::StandardCommandBufferAllocator;
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, RenderPassBeginInfo, SubpassBeginInfo};
use vulkano::shader::{ShaderModule, ShaderModuleCreateInfo};
use vulkano::sync::GpuFuture;

use winit::event_loop::EventLoop;
use winit::window::Window;

use crate::vertex::MyVertex;
use crate::mesh::MeshData;
use crate::renderer_config::RendererConfig;

#[repr(C)]
#[derive(BufferContents, Clone, Copy)]
pub struct ObjectPushConstant {  
    pub mvp: [[f32; 4]; 4],
    pub color: [f32; 4],
}

pub struct VulkanRenderer {
    queue: Arc<Queue>,
    swapchain: Arc<Swapchain>,
    framebuffers: Vec<Arc<Framebuffer>>,
    pipeline: Arc<GraphicsPipeline>,
    vertex_buffer: Subbuffer<[MyVertex]>,
    index_buffer: Subbuffer<[u32]>,
    index_count: u32,
    command_buffer_allocator: Arc<StandardCommandBufferAllocator>,
    clear_color: [f32; 4], 
}

impl VulkanRenderer {
    // We return a Result now. The `?` passes the error up to main.rs safely.
    pub fn new(event_loop: &EventLoop<()>, window: Arc<Window>, mesh_data: MeshData, config: RendererConfig) -> Result<Self, Box<dyn Error>> {
        let library = VulkanLibrary::new()?;
        let instance = Instance::new(library, InstanceCreateInfo {
            enabled_extensions: Surface::required_extensions(event_loop),
            ..Default::default()
        })?;

        let surface = Surface::from_window(instance.clone(), window.clone())?;
        let device_extensions = DeviceExtensions { khr_swapchain: true, ..DeviceExtensions::empty() };
        
        // ok_or converts an empty Option into a safe Error
        let (physical_device, queue_family_index) = instance.enumerate_physical_devices()?
            .filter(|p| p.supported_extensions().contains(&device_extensions))
            .filter_map(|p| {
                p.queue_family_properties().iter().enumerate()
                    .position(|(i, q)| q.queue_flags.contains(QueueFlags::GRAPHICS) && p.surface_support(i as u32, &surface).unwrap_or(false))
                    .map(|i| (p, i as u32))
            }).next().ok_or("No suitable physical device found")?;

        let (device, mut queues) = Device::new(physical_device, DeviceCreateInfo {
            queue_create_infos: vec![QueueCreateInfo { queue_family_index, ..Default::default() }],
            enabled_extensions: device_extensions,
            ..Default::default()
        })?;

        let queue = queues.next().ok_or("Failed to get device queue")?;
        let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));

        let (swapchain, images) = {
            let caps = device.physical_device().surface_capabilities(&surface, Default::default())?;
            let format = device.physical_device().surface_formats(&surface, Default::default())?[0].0;
            Swapchain::new(device.clone(), surface.clone(), SwapchainCreateInfo {
                min_image_count: caps.min_image_count,
                image_format: format,
                image_extent: window.inner_size().into(),
                image_usage: ImageUsage::COLOR_ATTACHMENT,
                ..Default::default()
            })?
        };

        let vertex_buffer = Buffer::from_iter(
            memory_allocator.clone(),
            BufferCreateInfo { usage: BufferUsage::VERTEX_BUFFER, ..Default::default() },
            AllocationCreateInfo { memory_type_filter: MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE, ..Default::default() },
            mesh_data.vertices,
        )?;

        let index_count = mesh_data.indices.len() as u32;
        let index_buffer = Buffer::from_iter(
            memory_allocator.clone(),
            BufferCreateInfo { usage: BufferUsage::INDEX_BUFFER, ..Default::default() },
            AllocationCreateInfo { memory_type_filter: MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE, ..Default::default() },
            mesh_data.indices,
        )?;

        let vs_words = vulkano::shader::spirv::bytes_to_words(config.vertex_shader_bytes)?;
        let fs_words = vulkano::shader::spirv::bytes_to_words(config.fragment_shader_bytes)?;
        let vs = unsafe { ShaderModule::new(device.clone(), ShaderModuleCreateInfo::new(&vs_words))? };
        let fs = unsafe { ShaderModule::new(device.clone(), ShaderModuleCreateInfo::new(&fs_words))? };

        let render_pass = vulkano::single_pass_renderpass!(device.clone(),
            attachments: { color: { format: swapchain.image_format(), samples: 1, load_op: Clear, store_op: Store, } },
            pass: { color: [color], depth_stencil: {} }
        )?;

        let subpass = Subpass::from(render_pass.clone(), 0).ok_or("Failed to create subpass")?;
        
        let vs_entry = vs.entry_point("main").ok_or("Failed to find main entry point in vertex shader")?;
        let fs_entry = fs.entry_point("main").ok_or("Failed to find main entry point in fragment shader")?;
        
        let push_constant_ranges = vs_entry.info().push_constant_requirements.into_iter().collect();

        let pipeline = GraphicsPipeline::new(device.clone(), None, GraphicsPipelineCreateInfo {
            stages: [
                PipelineShaderStageCreateInfo::new(vs_entry.clone()),
                PipelineShaderStageCreateInfo::new(fs_entry),
            ].into_iter().collect(), 
            vertex_input_state: Some(MyVertex::per_vertex().definition(&vs_entry.info().input_interface)?),
            input_assembly_state: Some(InputAssemblyState::default()),
            viewport_state: Some(ViewportState { 
                viewports: [Viewport { offset: [0.0, 0.0], extent: [config.viewport_width, config.viewport_height], depth_range: 0.0..=1.0 }].into_iter().collect(), 
                ..Default::default() 
            }),
            rasterization_state: Some(RasterizationState {
                cull_mode: config.cull_mode,
                front_face: FrontFace::CounterClockwise, 
                ..Default::default()
            }),
            multisample_state: Some(MultisampleState::default()),
            color_blend_state: Some(ColorBlendState::with_attachment_states(subpass.num_color_attachments(), ColorBlendAttachmentState::default())),
            subpass: Some(subpass.into()),
            ..GraphicsPipelineCreateInfo::layout(PipelineLayout::new(
                device.clone(), 
                PipelineLayoutCreateInfo {
                    push_constant_ranges,
                    ..Default::default()
                }
            )?)
        })?;

        let mut framebuffers = Vec::new();
        for img in images.iter() {
            let fb = Framebuffer::new(render_pass.clone(), FramebufferCreateInfo { 
                attachments: vec![ImageView::new_default(img.clone())?], 
                ..Default::default() 
            })?;
            framebuffers.push(fb);
        }

        let command_buffer_allocator = Arc::new(StandardCommandBufferAllocator::new(device.clone(), Default::default()));

        Ok(Self { 
            queue, 
            swapchain, 
            framebuffers, 
            pipeline, 
            vertex_buffer, 
            index_buffer, 
            index_count, 
            command_buffer_allocator,
            clear_color: config.clear_color
        })
    }

    pub fn draw(&mut self, mvp_matrix: [[f32; 4]; 4], color: [f32; 4]) -> Result<(), Box<dyn Error>> {
        // Swapchains frequently go out of date when windows resize. We handle it cleanly now!
        let (img_idx, _, acquire_future) = match vulkano::swapchain::acquire_next_image(self.swapchain.clone(), None) {
            Ok(r) => r,
            Err(_) => return Ok(()), // Silently skip drawing this frame if the window is resizing
        };

        let mut builder = AutoCommandBufferBuilder::primary(&*self.command_buffer_allocator, self.queue.queue_family_index(), CommandBufferUsage::OneTimeSubmit)?;
        let push_constant = ObjectPushConstant { mvp: mvp_matrix, color };

        builder.begin_render_pass(RenderPassBeginInfo { 
            clear_values: vec![Some(self.clear_color.into())], 
            ..RenderPassBeginInfo::framebuffer(self.framebuffers[img_idx as usize].clone())
        }, SubpassBeginInfo::default())?
        .bind_pipeline_graphics(self.pipeline.clone())?
        .bind_vertex_buffers(0, self.vertex_buffer.clone())?
        .bind_index_buffer(self.index_buffer.clone())?
        .push_constants(self.pipeline.layout().clone(), 0, push_constant)?
        .draw_indexed(self.index_count, 1, 0, 0, 0)?
        .end_render_pass(Default::default())?;

        acquire_future.then_execute(self.queue.clone(), builder.build()?)?
            .then_swapchain_present(self.queue.clone(), SwapchainPresentInfo::swapchain_image_index(self.swapchain.clone(), img_idx))
            .then_signal_fence_and_flush()?
            .wait(None)?;

        Ok(())
    }
}