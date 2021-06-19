use super::ray::Ray;
use super::vector::Vec3;

#[derive(Debug)]
pub struct Camera {
    origin: Vec3,
    corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Camera {
    /// New Camera for the Scene
    ///
    /// # Arguments
    ///
    /// - ``origin`` - the position of the Camera
    /// - ``look_at`` - direction of the Camera, the futher the better
    /// - ``fov`` - field of view in degrees, values between 50 and 90 work best
    /// - ``aspect_ratio`` - aspect ratio of the Scene (width/height)
    /// - ``roll`` - Camera rolling angle in degrees
    ///
    pub fn new(origin: Vec3, look_at: Vec3, fov: f64, aspect_ratio: f64, roll: f64) -> Camera {
        let roll_angle = roll.to_radians();
        let rotated_up = Vec3::new(-roll_angle.sin(), roll_angle.cos(), 0.0);

        let w = (origin - look_at).normalize();
        let u = rotated_up.cross(w).normalize();
        let v = w.cross(u).normalize();

        let half_height = (fov.to_radians() / 2.0).tan();
        let half_width = half_height * aspect_ratio;

        let corner = origin - (u * half_width) + (v * half_height) - w;
        let horizontal = u * (2.0 * half_width);
        let vertical = -v * (2.0 * half_height);

        Camera {
            origin,
            corner,
            horizontal,
            vertical,
            u,
            v,
            w,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let direction = self.corner + (self.horizontal * s) + (self.vertical * t) - self.origin;

        Ray {
            origin: self.origin,
            direction: direction.normalize(),
        }
    }
}
