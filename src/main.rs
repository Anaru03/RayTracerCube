mod camera;
mod cube;
mod framebuffer;
mod light;
mod ray;
mod texture;
mod vector;

use camera::Camera;
use cube::Cube;
use framebuffer::{Framebuffer, rgb};
use light::Light;
use minifb::{Key, Window, WindowOptions};
use texture::Texture;
use vector::Vec3;

const WIDTH: usize = 500;
const HEIGHT: usize = 500;
const ROTATION_SPEED: f32 = 0.05;

fn shade_rubik(ray: &ray::Ray, cube: &Cube, t: f32, light: &Light) -> u32 {
    let hit_point = ray.at(t);
    let normal = cube.normal_at(hit_point);

    let (face, u, v) = cube.texture_coordinates(hit_point);
    let texture_color = Texture::rubik(face, u, v);

    let r = ((texture_color >> 16) & 255) as f32;
    let g = ((texture_color >> 8) & 255) as f32;
    let b = (texture_color & 255) as f32;

    let diffuse = light.illuminate(hit_point, normal);

    let ambient = 0.20;
    let brightness = (ambient + diffuse * 0.80).min(1.0);

    rgb(
        (r * brightness) as u32,
        (g * brightness) as u32,
        (b * brightness) as u32,
    )
}

fn shade_floor(ray: &ray::Ray, floor: &Cube, t: f32, light: &Light) -> u32 {
    let hit_point = ray.at(t);
    let normal = floor.normal_at(hit_point);

    let diffuse = light.illuminate(hit_point, normal);

    let ambient = 0.20;
    let brightness = (ambient + diffuse * 0.80).min(1.0);

    rgb(
        (110.0 * brightness) as u32,
        (115.0 * brightness) as u32,
        (125.0 * brightness) as u32,
    )
}

fn render(
    framebuffer: &mut Framebuffer,
    cube: &Cube,
    floor: &Cube,
    camera: &Camera,
    light: &Light,
) {
    let background = rgb(18, 20, 25);

    framebuffer.clear(background);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let ray = camera.get_ray(x, y, WIDTH, HEIGHT);

            let cube_hit = cube.intersect(&ray);
            let floor_hit = floor.intersect(&ray);

            let color = match (cube_hit, floor_hit) {
                (Some(cube_t), Some(floor_t)) => {
                    if cube_t < floor_t {
                        shade_rubik(&ray, cube, cube_t, light)
                    } else {
                        shade_floor(&ray, floor, floor_t, light)
                    }
                }

                (Some(cube_t), None) => shade_rubik(&ray, cube, cube_t, light),

                (None, Some(floor_t)) => shade_floor(&ray, floor, floor_t, light),

                (None, None) => background,
            };

            framebuffer.set_pixel(x, y, color);
        }
    }
}

fn main() {
    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);

    let cube = Cube::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0));

    let floor = Cube::new(Vec3::new(-6.0, -1.25, -6.0), Vec3::new(6.0, -1.05, 6.0));

    let light = Light::new(Vec3::new(-3.0, 5.0, 4.0), 1.0);

    let mut camera = Camera::new(
        Vec3::new(4.0, 3.0, 5.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        60.0_f32.to_radians(),
    );

    let mut window = Window::new(
        "RayTracerCube | Rubik | Flechas: camara orbital | ESC: salir",
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

        render(&mut framebuffer, &cube, &floor, &camera, &light);

        window
            .update_with_buffer(&framebuffer.buffer, WIDTH, HEIGHT)
            .expect("No se pudo actualizar la ventana");
    }
}
