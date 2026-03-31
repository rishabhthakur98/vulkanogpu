use vulkano::buffer::BufferContents;
use vulkano::pipeline::graphics::vertex_input::Vertex;

#[derive(BufferContents, Vertex, Clone, Copy)]
#[repr(C)]
pub struct MyVertex {
    #[format(R32G32B32_SFLOAT)]
    pub position: [f32; 3],
}

impl MyVertex {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { position: [x, y, z] }
    }
}