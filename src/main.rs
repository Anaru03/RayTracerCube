mod camera;
mod cube;
mod framebuffer;
mod light;
mod ray;
mod vector;

use camera::Camera;
use cube::Cube;
use framebuffer::{Framebuffer, rgb};
use light::Light;
use minifb::{Key, Window, WindowOptions};
use vector::Vec3;

const WIDTH: usize = 500;
const HEIGHT: usize = 500;

const ROTATION_SPEED: f32 = 0.05;

fn render(framebuffer: &mut Framebuffer, cube: &Cube, camera: &Camera, light: &Light) {
    framebuffer.clear(rgb(18, 20, 25));

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let ray = camera.get_ray(x, y, WIDTH, HEIGHT);

            let color = if let Some(t) = cube.intersect(&ray) {
                let hit_point = ray.at(t);
                let normal = cube.normal_at(hit_point);

                let diffuse = light.illuminate(hit_point, normal);

                let ambient = 0.20;
                let brightness = (ambient + diffuse * 0.80).min(1.0);

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
}

fn main() {
    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);

    // Cubo fijo en el centro de la escena
    let cube = Cube::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0));

    // Luz fija en el mundo
    let light = Light::new(Vec3::new(-3.0, 5.0, 4.0), 1.0);

    // Camara orbital
    let mut camera = Camera::new(
        Vec3::new(4.0, 3.0, 5.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        60.0_f32.to_radians(),
    );

    let mut window = Window::new(
        "RayTracerCube | Flechas: camara orbital | ESC: salir",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .expect("No se pudo crear la ventana");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let orbit_controls = [
            (Key::Left, ROTATION_SPEED, 0.0),
            (Key::Right, -ROTATION_SPEED, 0.0),
            (Key::Up, 0.0, -ROTATION_SPEED),
            (Key::Down, 0.0, ROTATION_SPEED),
        ];

        for (key, delta_yaw, delta_pitch) in orbit_controls {
            if window.is_key_down(key) {
                camera.orbit(delta_yaw, delta_pitch);
            }
        }

        render(&mut framebuffer, &cube, &camera, &light);

        window
            .update_with_buffer(&framebuffer.buffer, WIDTH, HEIGHT)
            .expect("No se pudo actualizar la ventana");
    }
}
