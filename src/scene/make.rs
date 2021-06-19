use crate::color::Color;
use crate::intersectable::plane::Plane;
use crate::intersectable::sphere::Sphere;
use crate::light::Light;
use crate::light::LightType;
use crate::material::Material;
use crate::vector::Vec3;

use super::Scene;

impl Scene {
    /// Adds a new Sphere to the Scene
    ///
    /// # Arguments
    ///
    /// - ``pos`` - position of the center of the sphere
    /// - ``radius`` - radius of the sphere
    /// - ``color`` - material color
    /// - ``diffuse`` - amount of light diffused, defaults to 0.6 [0. 1.]
    /// - ``specular`` - specular coeff, defaults to 50.0
    /// - ``specular_exponent`` - specular exponent, defaults to 100.0
    /// - ``relectiveness`` - amount of light reflected, defaults to 1.0 [0. 1.]
    pub fn create_sphere(
        &mut self,
        pos: (f64, f64, f64),
        radius: f64,
        color: (u8, u8, u8),
        diffuse: Option<f64>,
        specular: Option<f64>,
        specular_exponent: Option<f64>,
        reflectiveness: Option<f64>,
    ) {
        let sphere = Sphere {
            position: Vec3::new(pos.0, pos.1, pos.2),
            radius,
            material: Material {
                color: Color::from_u8(color.0, color.1, color.2),
                diffuse: diffuse.unwrap_or(0.6),
                specular: specular.unwrap_or(50.0),
                specular_exponent: specular_exponent.unwrap_or(100.0),
                reflectiveness: reflectiveness.unwrap_or(1.0),
            },
        };
        self.objects.push(Box::new(sphere));
    }

    /// Adds a new Plane to the Scene
    ///
    /// # Arguments
    ///
    /// - ``pos`` - position of the center of the plane
    /// - ``normal`` - normal vector (perpendicular of the surface)
    /// - ``color`` - material color
    /// - ``diffuse`` - amount of light diffused, defaults to 0.6 [0. 1.]
    /// - ``specular`` - specular coeff, defaults to 50.0
    /// - ``specular_exponent`` - specular exponent, defaults to 100.0
    /// - ``relectiveness`` - amount of light reflected, defaults to 1.0 [0. 1.]
    pub fn create_plane(
        &mut self,
        pos: (f64, f64, f64),
        norm: (f64, f64, f64),
        color: (u8, u8, u8),
        diffuse: Option<f64>,
        specular: Option<f64>,
        specular_exponent: Option<f64>,
        reflectiveness: Option<f64>,
    ) {
        let plane = Plane {
            position: Vec3::new(pos.0, pos.1, pos.2),
            normal: Vec3::new(norm.0, norm.1, norm.2),
            material: Material {
                color: Color::from_u8(color.0, color.1, color.2),
                diffuse: diffuse.unwrap_or(0.6),
                specular: specular.unwrap_or(50.0),
                specular_exponent: specular_exponent.unwrap_or(100.0),
                reflectiveness: reflectiveness.unwrap_or(1.0),
            },
        };
        self.objects.push(Box::new(plane));
    }

    /// Adds a point Light to the Scene
    ///
    /// # Arguments
    ///
    /// - ``pos`` - position of the light
    /// - ``intensity`` - intensity [0. 1.]
    /// - ``color`` - color of the light
    pub fn create_point_light(
        &mut self,
        pos: (f64, f64, f64),
        intensity: f64,
        color: (u8, u8, u8),
    ) {
        let light = Light {
            light_type: LightType::Point,
            position: Vec3::new(pos.0, pos.1, pos.2),
            intensity,
            color: Color::from_u8(color.0, color.1, color.2),
        };
        self.lights.push(light);
    }

    /// Adds an ambient Light to the Scene
    ///
    /// # Arguments
    ///
    /// - ``intensity`` - intensity [0. 1.], defaults to 0.25
    /// - ``color`` - color of the light, defaults to white
    pub fn create_ambient_light(&mut self, intensity: Option<f64>, color: Option<(u8, u8, u8)>) {
        let c = color.unwrap_or((0xff, 0xff, 0xff));
        let light = Light {
            light_type: LightType::Ambient,
            position: Vec3::zero(),
            intensity: intensity.unwrap_or(0.25),
            color: Color::from_u8(c.0, c.1, c.2),
        };
        self.lights.push(light);
    }
}
