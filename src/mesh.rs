use crate::vertex::MyVertex;

pub struct MeshData {
    pub vertices: Vec<MyVertex>,
    pub indices: Vec<u32>,
}

impl MeshData {
    pub fn create_scene() -> Self {
        // 1. DEFINE YOUR UNIQUE POINTS IN SPACE [X, Y, Z]
        let vertices = vec![
            MyVertex::new(-0.5,  0.5, 0.0), // 0: Bottom Left
            MyVertex::new( 0.5,  0.5, 0.0), // 1: Bottom Right
            MyVertex::new( 0.0, -0.5, 0.0), // 2: Top Center
            MyVertex::new( 1.5, -0.5, 0.0), // 3: Top Right (Far away)
            MyVertex::new( 1.0,  0.5, 0.0), // 4: Bottom Right (Far away)
        ];

        // 2. GROUP THEM INTO TRIANGLES (Using their ID number from the list above)
        let indices = vec![
            0, 1, 2, // Triangle 1: Uses Bottom Left, Bottom Right, Top Center
            2, 1, 4, // Triangle 2: Connects the first triangle to the far bottom right
            2, 4, 3, // Triangle 3: Connects to make a bigger shape!
        ];

        Self { vertices, indices }
    }
}