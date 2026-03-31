mod vertex;
mod mesh;
mod camera_config; // Import the new camera module
mod renderer; 

use std::sync::Arc;
use winit::event::{Event, WindowEvent, KeyboardInput, VirtualKeyCode, ElementState};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{WindowBuilder, Fullscreen};

use renderer::VulkanRenderer;
use mesh::MeshData;
use camera_config::CameraConfig;
use vulkano::pipeline::graphics::rasterization::CullMode;

fn main() {
    let event_loop = EventLoop::new(); 
    
    let window = Arc::new(WindowBuilder::new()
        .with_inner_size(winit::dpi::PhysicalSize::new(1366, 768))
        .with_fullscreen(Some(Fullscreen::Borderless(None)))
        .build(&event_loop).unwrap());
    
    // 1. Fetch your custom shape
    let scene_data = MeshData::create_scene();

    // 2. Set the culling (None = double-sided)
    let cull_mode = CullMode::None;

    // 3. Initialize the Camera
    let camera = CameraConfig::new(1366.0, 768.0);

    // 4. Initialize the Engine
    let mut renderer = VulkanRenderer::new(&event_loop, window.clone(), scene_data, cull_mode);

    // 5. Run the continuous game loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput {
                    input: KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        state: ElementState::Pressed,
                        ..
                    },
                    ..
                } => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::MainEventsCleared => {
                // Get the matrix from the camera and pass it to the renderer!
                renderer.draw(camera.get_mvp_matrix());
            }
            _ => (),
        }
    });
}