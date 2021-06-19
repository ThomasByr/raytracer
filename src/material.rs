use super::color::Color;

#[derive(Clone, Copy, Debug)]
/// Generic Material
///
/// # Fields
///
/// - ``color`` - base color
/// - ``diffuse`` - diffuse coefficient
/// - ``specular`` - specular coefficient
/// - ``specular_exponent`` - specular exponent or shininess
/// - ``reflectiveness`` - reflectiveness, must be between 0. and 1.
pub struct Material {
    pub color: Color,
    pub diffuse: f64,
    pub specular: f64,
    pub specular_exponent: f64,
    pub reflectiveness: f64,
}

impl Material {
    pub fn neutral() -> Material {
        Material {
            color: Color::black(),
            diffuse: 0.0,
            specular: 0.0,
            specular_exponent: 0.0,
            reflectiveness: 0.0,
        }
    }
}
