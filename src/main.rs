mod cube;
mod ray;
mod vector;

use cube::Cube;
use ray::Ray;
use vector::Vec3;

fn main() {
    // Crear un cubo centrado en el origen
    let cube = Cube::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0));

    // Crear un rayo frente al cubo
    let ray = Ray::new(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, -1.0));

    println!("RayTracerCube");

    if let Some(t) = cube.intersect(&ray) {
        let hit_point = ray.at(t);
        let normal = cube.normal_at(hit_point);

        println!("El rayo golpeo el cubo");
        println!("Distancia t: {}", t);
        println!("Punto de impacto: {:?}", hit_point);
        println!("Normal: {:?}", normal);
    } else {
        println!("El rayo no golpeo el cubo");
    }
}
