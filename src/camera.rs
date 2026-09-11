use crate::ray::Ray;
use crate::vector::Vec3;

pub struct Camera {
    pub position: Vec3,
    pub target: Vec3,
    pub fov: f32,
}

impl Camera {
    pub fn new(position: Vec3, target: Vec3, fov: f32) -> Self {
        Self {
            position,
            target,
            fov,
        }
    }

    pub fn get_ray(&self, x: usize, y: usize, width: usize, height: usize) -> Ray {
        let aspect_ratio = width as f32 / height as f32;
        let fov_scale = (self.fov / 2.0).tan();

        let pixel_x = (2.0 * (x as f32 + 0.5) / width as f32 - 1.0) * aspect_ratio * fov_scale;

        let pixel_y = (1.0 - 2.0 * (y as f32 + 0.5) / height as f32) * fov_scale;

        let forward = (self.target - self.position).normalize();
        let world_up = Vec3::new(0.0, 1.0, 0.0);

        let right = forward.cross(&world_up).normalize();
        let up = right.cross(&forward).normalize();

        let direction = (forward + right * pixel_x + up * pixel_y).normalize();

        Ray::new(self.position, direction)
    }
}
