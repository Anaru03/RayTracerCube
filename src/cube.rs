use crate::ray::Ray;
use crate::texture::CubeFace;
use crate::vector::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Cube {
    pub min: Vec3,
    pub max: Vec3,
}

impl Cube {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<f32> {
        let mut t_min = (self.min.x - ray.origin.x) / ray.direction.x;
        let mut t_max = (self.max.x - ray.origin.x) / ray.direction.x;

        if t_min > t_max {
            std::mem::swap(&mut t_min, &mut t_max);
        }

        let mut ty_min = (self.min.y - ray.origin.y) / ray.direction.y;
        let mut ty_max = (self.max.y - ray.origin.y) / ray.direction.y;

        if ty_min > ty_max {
            std::mem::swap(&mut ty_min, &mut ty_max);
        }

        if t_min > ty_max || ty_min > t_max {
            return None;
        }

        t_min = t_min.max(ty_min);
        t_max = t_max.min(ty_max);

        let mut tz_min = (self.min.z - ray.origin.z) / ray.direction.z;
        let mut tz_max = (self.max.z - ray.origin.z) / ray.direction.z;

        if tz_min > tz_max {
            std::mem::swap(&mut tz_min, &mut tz_max);
        }

        if t_min > tz_max || tz_min > t_max {
            return None;
        }

        t_min = t_min.max(tz_min);
        t_max = t_max.min(tz_max);

        if t_min > 0.0 {
            Some(t_min)
        } else if t_max > 0.0 {
            Some(t_max)
        } else {
            None
        }
    }

    pub fn normal_at(&self, point: Vec3) -> Vec3 {
        let epsilon = 0.001;

        if (point.x - self.min.x).abs() < epsilon {
            Vec3::new(-1.0, 0.0, 0.0)
        } else if (point.x - self.max.x).abs() < epsilon {
            Vec3::new(1.0, 0.0, 0.0)
        } else if (point.y - self.min.y).abs() < epsilon {
            Vec3::new(0.0, -1.0, 0.0)
        } else if (point.y - self.max.y).abs() < epsilon {
            Vec3::new(0.0, 1.0, 0.0)
        } else if (point.z - self.min.z).abs() < epsilon {
            Vec3::new(0.0, 0.0, -1.0)
        } else {
            Vec3::new(0.0, 0.0, 1.0)
        }
    }

    pub fn texture_coordinates(&self, point: Vec3) -> (CubeFace, f32, f32) {
        let epsilon = 0.001;

        let width = self.max.x - self.min.x;
        let height = self.max.y - self.min.y;
        let depth = self.max.z - self.min.z;

        if (point.z - self.max.z).abs() < epsilon {
            let u = (point.x - self.min.x) / width;
            let v = 1.0 - (point.y - self.min.y) / height;

            (CubeFace::Front, u, v)
        } else if (point.z - self.min.z).abs() < epsilon {
            let u = 1.0 - (point.x - self.min.x) / width;
            let v = 1.0 - (point.y - self.min.y) / height;

            (CubeFace::Back, u, v)
        } else if (point.x - self.min.x).abs() < epsilon {
            let u = 1.0 - (point.z - self.min.z) / depth;
            let v = 1.0 - (point.y - self.min.y) / height;

            (CubeFace::Left, u, v)
        } else if (point.x - self.max.x).abs() < epsilon {
            let u = (point.z - self.min.z) / depth;
            let v = 1.0 - (point.y - self.min.y) / height;

            (CubeFace::Right, u, v)
        } else if (point.y - self.max.y).abs() < epsilon {
            let u = (point.x - self.min.x) / width;
            let v = (point.z - self.min.z) / depth;

            (CubeFace::Top, u, v)
        } else {
            let u = (point.x - self.min.x) / width;
            let v = 1.0 - (point.z - self.min.z) / depth;

            (CubeFace::Bottom, u, v)
        }
    }
}
