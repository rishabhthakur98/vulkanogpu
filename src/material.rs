pub struct Material {
    pub base_color: [f32; 4],
}

impl Default for Material {
    fn default() -> Self {
        Self {
            base_color: [1.0, 1.0, 1.0, 1.0], 
        }
    }
}