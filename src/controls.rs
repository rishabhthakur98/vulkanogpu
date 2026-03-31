use winit::event::VirtualKeyCode;

// We define an enum to tell the main loop what action to take
pub enum AppAction {
    Exit,
    Continue,
}

pub fn handle_keyboard(keycode: VirtualKeyCode) -> AppAction {
    match keycode {
        // If Escape is pressed, tell the app to exit
        VirtualKeyCode::Escape => AppAction::Exit,
        
        // You can add VirtualKeyCode::W here later for moving forward!
        
        // If it's any other key, just keep going
        _ => AppAction::Continue,
    }
}