mod vertex;
mod mesh;
mod camera_config; 
mod renderer_config; 
mod window_config; 
mod controls;     
mod renderer; 

use std::sync::Arc;
use std::time::{Instant, Duration}; 
use winit::event::{Event, WindowEvent, KeyboardInput, ElementState};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{WindowBuilder, Fullscreen};

use renderer::VulkanRenderer;
use mesh::MeshData;
use camera_config::CameraConfig;
use renderer_config::RendererConfig;
use window_config::WindowConfig;
use controls::{handle_keyboard, AppAction}; // Import the control handler

fn main() {
    let event_loop = EventLoop::new(); 
    
    // 1. Load Configurations
    let win_cfg = WindowConfig::default();
    let render_cfg = RendererConfig::default();
    
    // 2. Build the Window using window_config
    let fullscreen_mode = if win_cfg.fullscreen {
        Some(Fullscreen::Borderless(None))
    } else {
        None
    };

    let window = Arc::new(WindowBuilder::new()
        .with_inner_size(winit::dpi::PhysicalSize::new(win_cfg.width, win_cfg.height))
        .with_fullscreen(fullscreen_mode)
        .build(&event_loop).unwrap());
    
    // 3. Setup Scene, Camera, and Renderer
    let scene_data = MeshData::create_scene();
    let camera = CameraConfig::new(win_cfg.width as f32, win_cfg.height as f32);
    let mut renderer = VulkanRenderer::new(&event_loop, window.clone(), scene_data, render_cfg);

    // 4. Setup Frame Limiter (Calculates how long a single frame should take)
    let frame_duration = Duration::from_secs_f32(1.0 / win_cfg.target_fps as f32);
    let mut next_frame_time = Instant::now();

    // 5. Run the loop
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                
                // --- DELEGATE TO CONTROLS.RS ---
                WindowEvent::KeyboardInput {
                    input: KeyboardInput {
                        virtual_keycode: Some(keycode),
                        state: ElementState::Pressed,
                        ..
                    },
                    ..
                } => {
                    // We ask our controls file what to do with this key
                    match handle_keyboard(keycode) {
                        AppAction::Exit => *control_flow = ControlFlow::Exit,
                        AppAction::Continue => (),
                    }
                },
                _ => (),
            },
            
            // --- FRAME LIMITER LOGIC ---
            Event::MainEventsCleared => {
                let now = Instant::now();
                
                // If enough time has passed to hit our FPS target, draw a frame!
                if now >= next_frame_time {
                    renderer.draw(camera.get_mvp_matrix());
                    
                    // Set the alarm for the next frame
                    next_frame_time = now + frame_duration;
                }
                
                // Put the program to sleep until it is exactly time to draw again.
                // This saves massive CPU/GPU power!
                *control_flow = ControlFlow::WaitUntil(next_frame_time);
            }
            _ => (),
        }
    });
}