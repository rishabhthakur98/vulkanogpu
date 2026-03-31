pub struct WindowConfig {
    pub width: u32,
    pub height: u32,
    pub fullscreen: bool,
    pub target_fps: u32, 
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            width: 1366,
            height: 768,
            fullscreen: true,
            target_fps: 60, 
        }
    }
}