use vulkano::pipeline::graphics::rasterization::CullMode;

pub struct RendererConfig {
    pub vertex_shader_bytes: &'static [u8],
    pub fragment_shader_bytes: &'static [u8],
    pub viewport_width: f32,
    pub viewport_height: f32,
    pub clear_color: [f32; 4],
    pub cull_mode: CullMode,
}

impl Default for RendererConfig {
    fn default() -> Self {
        Self {
            // Your SPV files
            vertex_shader_bytes: include_bytes!("../glsl_files/vert.spv"),
            fragment_shader_bytes: include_bytes!("../glsl_files/frag.spv"),
            
            // Your screen dimensions
            viewport_width: 1366.0,
            viewport_height: 768.0,
            
            // Background color (R, G, B, Alpha) -> 0.1 is dark gray
            clear_color: [0.1, 0.1, 0.1, 1.0],
            
            // Double-sided by default
            cull_mode: CullMode::None,
        }
    }
}