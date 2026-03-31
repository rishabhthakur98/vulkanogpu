use cgmath::{Matrix4, Point3, Vector3, perspective, Deg, Rad, SquareMatrix}; 

pub struct CameraConfig {
    pub position: Point3<f32>,
    pub target: Point3<f32>,
    pub up: Vector3<f32>,
    pub aspect_ratio: f32,
    // Renamed to be explicitly clear that humans should type degrees here
    pub fov_degrees: f32, 
}

impl CameraConfig {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            position: Point3::new(0.0, 0.0, 2.0), 
            target: Point3::new(0.0, 0.0, 0.0),   
            up: Vector3::new(0.0, -1.0, 0.0),     
            aspect_ratio: width / height,
            
            // Setting it in degrees (45° or 90° is standard)
            fov_degrees: 45.0, 
        }
    }

    pub fn get_mvp_matrix(&self) -> [[f32; 4]; 4] {
        // --- THE CONVERSION ---
        // We take the human-readable degrees and mathematically convert them to radians
        let fov_radians: Rad<f32> = Rad::from(Deg(self.fov_degrees));
        
        // Now we pass the mathematically pure radians into the matrix calculator
        let proj = perspective(fov_radians, self.aspect_ratio, 0.01, 100.0);
        let view = Matrix4::look_at_rh(self.position, self.target, self.up);
        let model = Matrix4::identity(); // Keep the object at 0,0,0

        let mvp = proj * view * model;
        mvp.into() 
    }
}