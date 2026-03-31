use crate::vertex::MyVertex;

pub struct MeshData {
    pub vertices: Vec<MyVertex>,
    pub indices: Vec<u32>,
}

impl MeshData {
    pub fn create_cube() -> Self {
        let vertices = vec![
            MyVertex::new(-0.5, -0.5,  0.5), // 0: Front Bottom Left
            MyVertex::new( 0.5, -0.5,  0.5), // 1: Front Bottom Right
            MyVertex::new( 0.5,  0.5,  0.5), // 2: Front Top Right
            MyVertex::new(-0.5,  0.5,  0.5), // 3: Front Top Left
            
            MyVertex::new(-0.5, -0.5, -0.5), // 4: Back Bottom Left
            MyVertex::new( 0.5, -0.5, -0.5), // 5: Back Bottom Right
            MyVertex::new( 0.5,  0.5, -0.5), // 6: Back Top Right
            MyVertex::new(-0.5,  0.5, -0.5), // 7: Back Top Left
        ];

        let indices = vec![
            0, 1, 2,  2, 3, 0, // Front
            1, 5, 6,  6, 2, 1, // Right
            7, 6, 5,  5, 4, 7, // Back
            4, 0, 3,  3, 7, 4, // Left
            4, 5, 1,  1, 0, 4, // Bottom
            3, 2, 6,  6, 7, 3, // Top
        ];

        Self { vertices, indices }
    }
}