use raytracer::camera::Camera;
use raytracer::color::Color;
use raytracer::options::Options;
use raytracer::scene::Scene;
use raytracer::vector::Vec3;

use std::time::Instant;

const WIDTH: u32 = 16_384;
const HEIGHT: u32 = 10_240;
const DEPTH: u8 = 255;
const NAME: &str = "[16K][18.10]result";

fn setup_scene() -> Scene {
    let options = Options {
        max_rays: DEPTH,
        gamma: 0.85,
        diffuse: true,
        specular: true,
        shadows: true,
        reflections: true,
    };

    let aspect_ratio = WIDTH as f64 / HEIGHT as f64;

    let mut scene = Scene {
        width: WIDTH,
        height: HEIGHT,
        camera: Camera::new(
            Vec3::new(0., -3., 5.),
            Vec3::new(0., 0., -100.),
            60.,
            aspect_ratio,
            45.,
        ),
        objects: vec![], // needs to load a scene
        lights: vec![],  // ...
        bg_color: Color::black(),
        options,
    };
    scene.load_example(); // loading
    scene
}

fn main() {
    let mut scene = setup_scene();
    let now = Instant::now();

    std::fs::create_dir_all("exports").expect("Unable to create directory");
    scene.render(NAME.to_string());

    let duration = now.elapsed();

    println!(
        "{}.{} seconds elapsed.",
        duration.as_secs(),
        u64::from(duration.subsec_millis())
    );
}
