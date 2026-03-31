mod vertex;
mod mesh;
mod camera_config; 
mod renderer_config; 
mod window_config; 
mod controls;      
mod renderer; 

use std::sync::Arc;
use std::time::{Instant, Duration}; 
use winit::event::{Event, WindowEvent, DeviceEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{WindowBuilder, Fullscreen, CursorGrabMode};

use renderer::VulkanRenderer;
use mesh::MeshData;
use camera_config::CameraConfig;
use renderer_config::RendererConfig;
use window_config::WindowConfig;
use controls::InputState;

fn main() {
    // 1. STARTUP & CONFIGURATION
    // The EventLoop is our direct pipeline to the Operating System. It listens for everything:
    // mouse clicks, window resizing, and keyboard presses.
    let event_loop = EventLoop::new(); 
    
    // We load our separated configurations. This keeps our main file clean and data-driven.
    let win_cfg = WindowConfig::default();
    let render_cfg = RendererConfig::default();
    
    // Determine if we should request fullscreen from the OS based on our config.
    let fullscreen_mode = if win_cfg.fullscreen { 
        Some(Fullscreen::Borderless(None)) 
    } else { 
        None 
    };

    // 2. WINDOW CREATION (Safely!)
    // Instead of using .unwrap() which violently crashes the program if it fails,
    // we use a `match` statement to handle the error gracefully.
    let window = match WindowBuilder::new()
        .with_inner_size(winit::dpi::PhysicalSize::new(win_cfg.width, win_cfg.height))
        .with_fullscreen(fullscreen_mode)
        .build(&event_loop) 
    {
        Ok(win) => Arc::new(win), // Wrap in Arc so multiple parts of the engine can share the window
        Err(e) => {
            // If the OS refuses to make a window (e.g., headless server, missing drivers),
            // we print a helpful error and exit cleanly instead of panicking.
            eprintln!("CRITICAL ERROR: Failed to create the OS Window. Details: {}", e);
            std::process::exit(1);
        }
    };
    
    // 3. MOUSE LOCKING (The Graceful Fallback)
    // In a 3D First-Person game, the mouse needs to be trapped so it doesn't click outside the window.
    // Wayland/macOS sometimes aggressively block this. Instead of crashing, we print a warning.
    if let Err(e) = window.set_cursor_grab(CursorGrabMode::Confined)
        .or_else(|_| window.set_cursor_grab(CursorGrabMode::Locked)) 
    {
        eprintln!("WARNING: The OS refused to lock the mouse to the window. You can still play, but your mouse might leave the screen! Details: {}", e);
    }
    window.set_cursor_visible(false); // Hide the actual mouse pointer arrow

    // 4. ENGINE INITIALIZATION
    // We fetch our 3D triangle data and initialize the camera mathematics.
    let scene_data = MeshData::create_scene();
    let mut camera = CameraConfig::new(win_cfg.width as f32, win_cfg.height as f32);
    
    // Initialize the massive Vulkan rendering pipeline
    let mut renderer = VulkanRenderer::new(&event_loop, window.clone(), scene_data, render_cfg);

    // Initialize our input buffer. This stores what keys are currently held down.
    let mut input_state = InputState::default();

    // 5. FRAME TIMING & PHYSICS PREPARATION
    // To make sure the game runs at exactly the speed we want (e.g., 60 FPS), we calculate 
    // exactly how long one frame should take in seconds.
    let frame_duration = Duration::from_secs_f32(1.0 / win_cfg.target_fps as f32);
    
    // We track time using the CPU's high-precision clock (`Instant::now()`).
    let mut next_frame_time = Instant::now();
    let mut last_frame_time = Instant::now();

    // 6. THE GAME LOOP
    // This loop runs continuously until the program is closed.
    event_loop.run(move |event, _, control_flow| {
        match event {
            // --- OS WINDOW EVENTS (Keyboard, Closing, Resizing) ---
            Event::WindowEvent { event, .. } => match event {
                // If the user presses the 'X' button or Alt+F4
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                
                // If the user touches the keyboard
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(keycode) = input.virtual_keycode {
                        // Pass the key to our controls.rs file to update our input buffer
                        input_state.process_keyboard(keycode, input.state);
                    }
                    // If controls.rs decided we pressed the Escape key, we exit.
                    if input_state.exit {
                        *control_flow = ControlFlow::Exit;
                    }
                },
                _ => (),
            },
            
            // --- RAW HARDWARE EVENTS (The Mouse) ---
            // DeviceEvent reads directly from the mouse hardware, ignoring the OS cursor position.
            // This is required for 3D camera rotation so it doesn't stop turning when the cursor hits the edge of the screen.
            Event::DeviceEvent { event: DeviceEvent::MouseMotion { delta }, .. } => {
                input_state.process_mouse(delta.0, delta.1);
            },
            
            // --- THE RENDER & UPDATE CYCLE ---
            // This event fires constantly when the OS has no other messages to process.
            Event::MainEventsCleared => {
                let now = Instant::now();
                
                // Have we waited long enough to draw the next frame? (The Frame Limiter)
                if now >= next_frame_time {
                    // Calculate Delta Time (dt): Exactly how much time passed since the last frame.
                    // This ensures moving 1 meter per second is identical on both slow and fast computers.
                    let dt = now.duration_since(last_frame_time).as_secs_f32();
                    last_frame_time = now;
                    
                    // 1. Update Game Physics/Math: Move the camera based on our inputs
                    camera.update(dt, &mut input_state);

                    // 2. Render Graphics: Send the new camera matrix to the GPU and draw the triangles
                    renderer.draw(camera.get_mvp_matrix());
                    
                    // 3. Reset the timer for the next frame
                    next_frame_time = now + frame_duration;
                }
                
                // SLEEP THE THREAD
                // Instead of burning 100% CPU power by constantly checking the time,
                // we tell the OS: "Put this program to sleep until it is exactly time for the next frame."
                *control_flow = ControlFlow::WaitUntil(next_frame_time);
            }
            _ => (),
        }
    });
}