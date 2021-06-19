use crate::color::Color;
use crate::intersectable::plane::Plane;
use crate::intersectable::sphere::Sphere;
use crate::light::Light;
use crate::light::LightType;
use crate::material::Material;
use crate::vector::Vec3;

use super::Scene;

impl Scene {
    /// Example of a Scene you can find under the ``examples`` folder
    pub fn load_example(&mut self) {
        self.objects = vec![
            Box::new(Sphere {
                position: Vec3::new(-3.0, -5.0, -16.0),
                radius: 2.8,
                material: Material {
                    color: Color::from_u8(0xff, 0x55, 0x55),
                    diffuse: 0.6,
                    specular: 50.0,
                    specular_exponent: 100.0,
                    reflectiveness: 0.0,
                },
            }),
            Box::new(Sphere {
                position: Vec3::new(0.0, -5.0, -13.0),
                radius: 2.0,
                material: Material {
                    color: Color::from_u8(0x40, 0xe0, 0xd0),
                    diffuse: 0.6,
                    specular: 5.0,
                    specular_exponent: 500.0,
                    reflectiveness: 0.0,
                },
            }),
            Box::new(Sphere {
                position: Vec3::new(3.0, -5.0, -17.0),
                radius: 2.8,
                material: Material {
                    color: Color::from_u8(0x77, 0xbb, 0x77),
                    diffuse: 0.5,
                    specular: 0.2,
                    specular_exponent: 2.0,
                    reflectiveness: 0.0,
                },
            }),
            Box::new(Sphere {
                position: Vec3::new(0.0, -4.0, -20.0),
                radius: 3.0,
                material: Material {
                    color: Color::from_u8(0x2f, 0x8d, 0xff),
                    diffuse: 0.6,
                    specular: 3.0,
                    specular_exponent: 50.0,
                    reflectiveness: 0.0,
                },
            }),
            Box::new(Sphere {
                position: Vec3::new(-10.0, 5.0, -20.0),
                radius: 5.0,
                material: Material {
                    color: Color::new(0.1, 0.1, 0.1),
                    diffuse: 0.0,
                    specular: 50.0,
                    specular_exponent: 100.0,
                    reflectiveness: 1.0,
                },
            }),
            Box::new(Plane {
                position: Vec3::new(0.0, -8.0, 0.0),
                normal: Vec3::new(0.0, -1.0, 0.0),
                material: Material {
                    color: Color::from_u8(0x66, 0x33, 0x66),
                    diffuse: 0.8,
                    specular: 0.2,
                    specular_exponent: 5.0,
                    reflectiveness: 0.6,
                },
            }),
        ];
        self.lights = vec![
            Light {
                light_type: LightType::Point,
                position: Vec3::new(-40.0, 20.0, 20.0),
                intensity: 1.0,
                color: Color::white(),
            },
            Light {
                light_type: LightType::Point,
                position: Vec3::new(40.0, 20.0, 20.0),
                intensity: 0.8,
                color: Color::new(0.66, 0.0, 0.66),
            },
            Light {
                light_type: LightType::Point,
                position: Vec3::new(00.0, 50.0, 0.0),
                intensity: 0.8,
                color: Color::from_u8(0xa6, 0x7c, 0x00),
            },
            Light {
                light_type: LightType::Ambient,
                position: Vec3::zero(),
                intensity: 0.25,
                color: Color::white(),
            },
        ];
    }

    /// Example of a Scene with mirrors
    pub fn load_mirrors(&mut self) {
        self.objects = vec![
            Box::new(Plane {
                position: Vec3::new(0., 0., -20.),
                normal: Vec3::new(0., 0., -1.),
                material: Material {
                    color: Color::from_u8(0xff, 0xff, 0xff),
                    diffuse: 0.01,
                    specular: 1.,
                    specular_exponent: 1000.,
                    reflectiveness: 1.,
                },
            }),
            Box::new(Plane {
                position: Vec3::new(0., 0., 20.),
                normal: Vec3::new(0., 0., 1.),
                material: Material {
                    color: Color::from_u8(0xff, 0xff, 0xff),
                    diffuse: 0.,
                    specular: 1.,
                    specular_exponent: 1000.,
                    reflectiveness: 1.,
                },
            }),
            Box::new(Sphere {
                position: Vec3::new(5., 1., 0.),
                radius: 2.5,
                material: Material {
                    color: Color::from_u8(0x00, 0x15, 0x55),
                    diffuse: 1.0,
                    specular: 1.,
                    specular_exponent: 1000.,
                    reflectiveness: 1.,
                },
            }),
            Box::new(Sphere {
                position: Vec3::new(0., 0., 20.),
                radius: 5.,
                material: Material {
                    color: Color::from_u8(0x00, 0x00, 0x00),
                    diffuse: 0.,
                    specular: 1.,
                    specular_exponent: 1000.,
                    reflectiveness: 1.,
                },
            }),
        ];
        self.lights = vec![
            Light {
                light_type: LightType::Ambient,
                position: Vec3::new(-10., 0., 0.),
                intensity: 1.,
                color: Color::white(),
            },
            Light {
                light_type: LightType::Point,
                position: Vec3::new(-10., 0., 0.),
                intensity: 1.,
                color: Color::white(),
            },
        ]
    }

    /// Example for Wallpaper1
    pub fn load_wallpaper1(&mut self) {
        self.objects = vec![
            Box::new(Sphere {
                position: Vec3::new(1.0, -5.0, -15.0),
                radius: 2.1,
                material: Material {
                    color: Color::from_u8(0x00, 0x50, 0x1a),
                    diffuse: 0.6,
                    specular: 50.0,
                    specular_exponent: 100.0,
                    reflectiveness: 1.0,
                },
            }),
            Box::new(Sphere {
                position: Vec3::new(0.0, -1.0, -20.0),
                radius: 1.9,
                material: Material {
                    color: Color::from_u8(0x00, 0x8d, 0xff),
                    diffuse: 0.6,
                    specular: 50.0,
                    specular_exponent: 100.0,
                    reflectiveness: 1.0,
                },
            }),
            Box::new(Sphere {
                position: Vec3::new(-1.0, 2.0, -15.0),
                radius: 0.9,
                material: Material {
                    color: Color::from_u8(0x66, 0x00, 0x00),
                    diffuse: 0.6,
                    specular: 50.,
                    specular_exponent: 100.0,
                    reflectiveness: 1.0,
                },
            }),
            Box::new(Sphere {
                position: Vec3::new(-5., 7., -20.0),
                radius: 5.0,
                material: Material {
                    color: Color::new(0.1, 0.1, 0.1),
                    diffuse: 0.0,
                    specular: 50.0,
                    specular_exponent: 100.0,
                    reflectiveness: 1.0,
                },
            }),
            Box::new(Plane {
                position: Vec3::new(0.0, -8.0, 0.0),
                normal: Vec3::new(0.0, -1.0, 0.0),
                material: Material {
                    color: Color::from_u8(0x00, 0x15, 0x55),
                    diffuse: 0.8,
                    specular: 0.2,
                    specular_exponent: 5.0,
                    reflectiveness: 0.6,
                },
            }),
        ];
        self.lights = vec![
            Light {
                light_type: LightType::Point,
                position: Vec3::new(-40.0, 15.0, 20.0),
                intensity: 1.,
                color: Color::from_u8(0x35, 0x35, 0x35),
            },
            Light {
                light_type: LightType::Point,
                position: Vec3::new(40.0, 15.0, 20.0),
                intensity: 0.8,
                color: Color::new(0.75, 0., 0.),
            },
            Light {
                light_type: LightType::Point,
                position: Vec3::new(0., 0., 0.75),
                intensity: 0.8,
                color: Color::from_u8(0xa6, 0x7c, 0x00),
            },
            Light {
                light_type: LightType::Ambient,
                position: Vec3::zero(),
                intensity: 0.25,
                color: Color::white(),
            },
        ];
    }

    /// Example for Wallpaper2
    pub fn load_wallpaper2(&mut self) {
        self.objects = vec![
            Box::new(Sphere {
                position: Vec3::new(-3.0, -5.0, -16.0),
                radius: 2.1,
                material: Material {
                    color: Color::from_u8(0x00, 0x50, 0x1a),
                    diffuse: 0.6,
                    specular: 50.0,
                    specular_exponent: 100.0,
                    reflectiveness: 1.0,
                },
            }),
            Box::new(Sphere {
                position: Vec3::new(4.0, -5.0, -15.0),
                radius: 2.1,
                material: Material {
                    color: Color::from_u8(0x66, 0x00, 0x00),
                    diffuse: 0.6,
                    specular: 50.0,
                    specular_exponent: 100.0,
                    reflectiveness: 1.0,
                },
            }),
            Box::new(Sphere {
                position: Vec3::new(0.0, -5.0, -21.0),
                radius: 2.1,
                material: Material {
                    color: Color::from_u8(0x00, 0x35, 0x75),
                    diffuse: 0.6,
                    specular: 50.,
                    specular_exponent: 100.0,
                    reflectiveness: 1.0,
                },
            }),
            Box::new(Sphere {
                position: Vec3::new(-5., 7., -20.0),
                radius: 5.0,
                material: Material {
                    color: Color::new(0.1, 0.1, 0.1),
                    diffuse: 0.0,
                    specular: 50.0,
                    specular_exponent: 100.0,
                    reflectiveness: 1.0,
                },
            }),
            Box::new(Plane {
                position: Vec3::new(0.0, -8.0, 0.0),
                normal: Vec3::new(0.0, -1.0, 0.0),
                material: Material {
                    color: Color::from_u8(0x00, 0x15, 0x55),
                    diffuse: 0.0,
                    specular: 50.0,
                    specular_exponent: 100.0,
                    reflectiveness: 1.0,
                },
            }),
        ];
        self.lights = vec![
            Light {
                light_type: LightType::Point,
                position: Vec3::new(-40.0, 15.0, 20.0),
                intensity: 1.,
                color: Color::from_u8(0xa0, 0xa0, 0xa0),
            },
            Light {
                light_type: LightType::Point,
                position: Vec3::new(40.0, 15.0, 20.0),
                intensity: 0.8,
                color: Color::from_u8(0xa0, 0x00, 0x00),
            },
            Light {
                light_type: LightType::Point,
                position: Vec3::new(0., 0., 0.75),
                intensity: 0.8,
                color: Color::from_u8(0x00, 0xa0, 0xa0),
            },
            Light {
                light_type: LightType::Ambient,
                position: Vec3::zero(),
                intensity: 0.25,
                color: Color::white(),
            },
        ];
    }
}
