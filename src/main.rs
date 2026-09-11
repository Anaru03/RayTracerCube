mod bmp;
mod camera;
mod cube;
mod framebuffer;
mod light;
mod ray;
mod vector;

use bmp::save_bmp;
use camera::Camera;
use cube::Cube;
use framebuffer::{Framebuffer, rgb};
use light::Light;
use vector::Vec3;

const WIDTH: usize = 500;
const HEIGHT: usize = 500;

fn main() {
    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);

    let cube = Cube::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0));

    let camera = Camera::new(
        Vec3::new(4.0, 3.0, 5.0),
        Vec3::new(0.0, 0.0, 0.0),
        60.0_f32.to_radians(),
    );

    let light = Light::new(Vec3::new(-3.0, 5.0, 4.0), 1.0);

    framebuffer.clear(rgb(18, 20, 25));

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let ray = camera.get_ray(x, y, WIDTH, HEIGHT);

            let color = if let Some(t) = cube.intersect(&ray) {
                let hit_point = ray.at(t);
                let normal = cube.normal_at(hit_point);

                let diffuse = light.illuminate(hit_point, normal);

                let ambient = 0.15;
                let brightness = (ambient + diffuse * 0.85).min(1.0);

                let base_r = 180.0;
                let base_g = 185.0;
                let base_b = 195.0;

                rgb(
                    (base_r * brightness) as u32,
                    (base_g * brightness) as u32,
                    (base_b * brightness) as u32,
                )
            } else {
                rgb(18, 20, 25)
            };

            framebuffer.set_pixel(x, y, color);
        }
    }

    save_bmp(&framebuffer, "cube.bmp").expect("No se pudo guardar el render");

    println!("RayTracerCube");
    println!("Render terminado");
    println!("Imagen guardada en cube.bmp");
}
