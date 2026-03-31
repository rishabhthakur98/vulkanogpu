use winit::event::{VirtualKeyCode, ElementState};

#[derive(Default)]
pub struct InputState {
    pub move_forward: bool,
    pub move_backward: bool,
    pub move_left: bool,
    pub move_right: bool,
    pub mouse_dx: f64,
    pub mouse_dy: f64,
    pub exit: bool,
}

impl InputState {
    pub fn process_keyboard(&mut self, keycode: VirtualKeyCode, state: ElementState) {
        // Is the key being pressed down, or was it just released?
        let pressed = state == ElementState::Pressed;
        
        match keycode {
            VirtualKeyCode::W => self.move_forward = pressed,
            VirtualKeyCode::S => self.move_backward = pressed,
            VirtualKeyCode::A => self.move_left = pressed,
            VirtualKeyCode::D => self.move_right = pressed,
            VirtualKeyCode::Escape => self.exit = pressed,
            _ => {}
        }
    }

    pub fn process_mouse(&mut self, dx: f64, dy: f64) {
        // Accumulate mouse movement until the camera processes it
        self.mouse_dx += dx;
        self.mouse_dy += dy;
    }

    pub fn reset_mouse(&mut self) {
        // Clear the mouse movement after the camera uses it
        self.mouse_dx = 0.0;
        self.mouse_dy = 0.0;
    }
}