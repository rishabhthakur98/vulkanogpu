use cgmath::{Matrix4, Point3, Vector3, perspective, Deg, Rad, InnerSpace};
use crate::controls::InputState;

pub struct CameraConfig {
    pub position: Point3<f32>,
    pub yaw: Rad<f32>,   
    pub pitch: Rad<f32>, 
    pub up: Vector3<f32>,
    pub aspect_ratio: f32,
    pub fov_degrees: f32,
    pub speed: f32,       
    pub sensitivity: f32, 
}

impl CameraConfig {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            position: Point3::new(0.0, 0.0, 2.0), 
            yaw: Rad(-std::f32::consts::FRAC_PI_2), 
            pitch: Rad(0.0),
            up: Vector3::new(0.0, -1.0, 0.0), 
            aspect_ratio: width / height,
            fov_degrees: 45.0,
            speed: 1.0, 
            sensitivity: 0.003, 
        }
    }

    pub fn update(&mut self, dt: f32, input: &mut InputState) {
        self.yaw += Rad(input.mouse_dx as f32 * self.sensitivity);
        self.pitch += Rad(input.mouse_dy as f32 * self.sensitivity);

        let half_pi = std::f32::consts::FRAC_PI_2 - 0.01;
        if self.pitch.0 > half_pi { self.pitch.0 = half_pi; }
        if self.pitch.0 < -half_pi { self.pitch.0 = -half_pi; }

        input.reset_mouse(); 

        let forward = Vector3::new(
            self.yaw.0.cos() * self.pitch.0.cos(),
            -self.pitch.0.sin(), 
            self.yaw.0.sin() * self.pitch.0.cos()
        ).normalize();

        let right = forward.cross(self.up).normalize();
        let velocity = self.speed * dt; 
        
        if input.move_forward { self.position += forward * velocity; }
        if input.move_backward { self.position -= forward * velocity; }
        if input.move_right { self.position += right * velocity; }
        if input.move_left { self.position -= right * velocity; }
    }

    pub fn get_projection_matrix(&self) -> Matrix4<f32> {
        let fov_radians: Rad<f32> = Rad::from(Deg(self.fov_degrees));
        perspective(fov_radians, self.aspect_ratio, 0.01, 100.0)
    }

    pub fn get_view_matrix(&self) -> Matrix4<f32> {
        let forward = Vector3::new(
            self.yaw.0.cos() * self.pitch.0.cos(),
            -self.pitch.0.sin(),
            self.yaw.0.sin() * self.pitch.0.cos()
        ).normalize();
        let target = self.position + forward;
        Matrix4::look_at_rh(self.position, target, self.up)
    }
}