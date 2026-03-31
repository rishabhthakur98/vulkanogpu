mod vertex;
mod mesh;
mod transform;
mod material;
mod game_object;
mod camera_config; 
mod renderer_config; 
mod window_config; 
mod controls;      
mod renderer; 

use std::sync::Arc;
use std::time::{Instant, Duration}; 
use winit::event::{Event, WindowEvent, DeviceEvent, KeyboardInput};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{WindowBuilder, Fullscreen, CursorGrabMode};

use renderer::VulkanRenderer;
use mesh::MeshData;
use game_object::GameObject;
use camera_config::CameraConfig;
use renderer_config::RendererConfig;
use window_config::WindowConfig;
use controls::InputState;

fn main() {
    let event_loop = EventLoop::new(); 
    let win_cfg = WindowConfig::default();
    let render_cfg = RendererConfig::default();
    
    let fullscreen_mode = if win_cfg.fullscreen { Some(Fullscreen::Borderless(None)) } else { None };

    // Gracefully handle OS Window failure
    let window = match WindowBuilder::new()
        .with_inner_size(winit::dpi::PhysicalSize::new(win_cfg.width, win_cfg.height))
        .with_fullscreen(fullscreen_mode)
        .build(&event_loop) 
    {
        Ok(win) => Arc::new(win),
        Err(e) => {
            eprintln!("CRITICAL ERROR: Failed to create Window. Details: {}", e);
            std::process::exit(1);
        }
    };
    
    if let Err(e) = window.set_cursor_grab(CursorGrabMode::Confined)
        .or_else(|_| window.set_cursor_grab(CursorGrabMode::Locked)) 
    {
        eprintln!("WARNING: OS prevented mouse lock. Details: {:?}", e);
    }
    window.set_cursor_visible(false); 

    let cube_mesh = MeshData::create_cube();
    let mut my_cube = GameObject::new(cube_mesh);
    my_cube.material.base_color = [0.2, 0.8, 0.2, 1.0]; // Bright Green!
    my_cube.transform.position.z = -2.0; 

    let mut camera = CameraConfig::new(win_cfg.width as f32, win_cfg.height as f32);
    
    // Gracefully handle Vulkan initialization failures (missing drivers, etc.)
    let mut renderer = match VulkanRenderer::new(&event_loop, window.clone(), my_cube.mesh, render_cfg) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("CRITICAL ERROR: Failed to initialize Vulkan GPU Engine. Details: {}", e);
            std::process::exit(1);
        }
    };

    let mut input_state = InputState::default();
    let frame_duration = Duration::from_secs_f32(1.0 / win_cfg.target_fps as f32);
    let mut next_frame_time = Instant::now();
    let mut last_frame_time = Instant::now(); 

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(keycode) = input.virtual_keycode {
                        input_state.process_keyboard(keycode, input.state);
                    }
                    if input_state.exit {
                        *control_flow = ControlFlow::Exit;
                    }
                },
                _ => (),
            },
            
            Event::DeviceEvent { event: DeviceEvent::MouseMotion { delta }, .. } => {
                input_state.process_mouse(delta.0, delta.1);
            },
            
            Event::MainEventsCleared => {
                let now = Instant::now();
                
                if now >= next_frame_time {
                    let dt = now.duration_since(last_frame_time).as_secs_f32();
                    last_frame_time = now;
                    
                    camera.update(dt, &mut input_state);

                    // Spin the cube!
                    my_cube.transform.rotation.y += 45.0 * dt;
                    my_cube.transform.rotation.x += 20.0 * dt;

                    let proj = camera.get_projection_matrix(); 
                    let view = camera.get_view_matrix();
                    let model = my_cube.transform.get_model_matrix();
                    
                    let mvp = proj * view * model;

                    // Safely execute the draw call. If it fails, log it and keep the engine running!
                    if let Err(e) = renderer.draw(mvp.into(), my_cube.material.base_color) {
                        eprintln!("RENDER ERROR: Frame dropped. Details: {}", e);
                    }
                    
                    next_frame_time = now + frame_duration;
                }
                
                *control_flow = ControlFlow::WaitUntil(next_frame_time);
            }
            _ => (),
        }
    });
}