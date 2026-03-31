use cgmath::{Matrix4, Point3, Vector3, perspective, Deg, Rad, SquareMatrix, InnerSpace};
use crate::controls::InputState;

pub struct CameraConfig {
    pub position: Point3<f32>,
    pub yaw: Rad<f32>,   // Looking left/right
    pub pitch: Rad<f32>, // Looking up/down
    pub up: Vector3<f32>,
    pub aspect_ratio: f32,
    pub fov_degrees: f32,
    pub speed: f32,       // Movement speed in meters per second
    pub sensitivity: f32, // Mouse sensitivity
}

impl CameraConfig {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            position: Point3::new(0.0, 0.0, 2.0), 
            // -90 degrees yaw makes us face straight down the Z axis
            yaw: Rad(-std::f32::consts::FRAC_PI_2), 
            pitch: Rad(0.0),
            up: Vector3::new(0.0, -1.0, 0.0), // Vulkan Y is inverted
            aspect_ratio: width / height,
            fov_degrees: 45.0,
            
            // Your requested settings!
            speed: 1.0, 
            sensitivity: 0.003, 
        }
    }

    // This runs every single frame
    pub fn update(&mut self, dt: f32, input: &mut InputState) {
        // 1. PROCESS MOUSE (ROTATION)
        self.yaw += Rad(input.mouse_dx as f32 * self.sensitivity);
        self.pitch += Rad(input.mouse_dy as f32 * self.sensitivity);

        // Clamp the pitch so the player can't backflip their neck and break the camera
        let half_pi = std::f32::consts::FRAC_PI_2 - 0.01;
        if self.pitch.0 > half_pi { self.pitch.0 = half_pi; }
        if self.pitch.0 < -half_pi { self.pitch.0 = -half_pi; }

        input.reset_mouse(); // We used the mouse data, clear it!

        // 2. CALCULATE DIRECTION VECTORS
        // Trigonometry to find out exactly which way "Forward" is based on where we are looking
        let forward = Vector3::new(
            self.yaw.0.cos() * self.pitch.0.cos(),
            -self.pitch.0.sin(), // Inverted for Vulkan's downward Y axis
            self.yaw.0.sin() * self.pitch.0.cos()
        ).normalize();

        // "Right" is calculated by crossing Forward with Up
        let right = forward.cross(self.up).normalize();

        // 3. PROCESS KEYBOARD (MOVEMENT)
        let velocity = self.speed * dt; // e.g. 1.0 meters * 0.016 seconds
        
        if input.move_forward { self.position += forward * velocity; }
        if input.move_backward { self.position -= forward * velocity; }
        if input.move_right { self.position += right * velocity; }
        if input.move_left { self.position -= right * velocity; }
    }

    pub fn get_mvp_matrix(&self) -> [[f32; 4]; 4] {
        let fov_radians: Rad<f32> = Rad::from(Deg(self.fov_degrees));
        let proj = perspective(fov_radians, self.aspect_ratio, 0.01, 100.0);
        
        // Calculate the target by looking 1 unit 'forward' from our current position
        let forward = Vector3::new(
            self.yaw.0.cos() * self.pitch.0.cos(),
            -self.pitch.0.sin(),
            self.yaw.0.sin() * self.pitch.0.cos()
        ).normalize();
        let target = self.position + forward;

        let view = Matrix4::look_at_rh(self.position, target, self.up);
        let model = Matrix4::identity();

        let mvp = proj * view * model;
        mvp.into() 
    }
}