use std::fs::File;
use std::io::{BufWriter, Write};

mod color;
mod ray;
mod vec3;

use color::{Color, write_color};
use ray::Ray;
use vec3::{Point3, Vec3, dot, unit_vector};

fn ray_color(ray: &Ray) -> Color {
    let t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, ray);

    if t > 0.0 {
        let n = unit_vector(ray.at(t) - Vec3::new(0.0, 0.0, -1.0));
        return 0.5 * Color::new(n.x + 1.0, n.y + 1.0, n.z + 1.0);
        //return Color::new(0.66, 0.66, 0.66);
    }

    let unit_direction = unit_vector(ray.direction);
    let a = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0);
}

fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = center - ray.origin;
    let a = dot(ray.direction, ray.direction);
    let b = -2.0 * dot(ray.direction, oc);
    let c = dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-b - discriminant.sqrt()) / (2.0 * a);
    };
}

fn main() {
    const IMAGE_WIDTH: i32 = 1920;
    const IMAGE_HEIGHT: i32 = 1080;
    const ASPECT_RATIO: f64 = 16.0 / 9.0;

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * ASPECT_RATIO;
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / IMAGE_WIDTH as f64;
    let pixel_delta_v = viewport_v / IMAGE_HEIGHT as f64;

    let focal_length = Vec3::new(0.0, 0.0, focal_length);
    let viewport_upper_left = camera_center - focal_length - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let file = File::create("ember.ppm").expect("Failed to create file");
    let mut writer = BufWriter::new(file);

    write!(writer, "P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).unwrap();

    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            let pixel_center = pixel00_loc + (i * pixel_delta_u) + (j * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&ray);
            write_color(&mut writer, pixel_color);
        }
    }
}
