use indicatif::ProgressBar;
use ray_tracing_in_one_weekend::ray::Ray;
use ray_tracing_in_one_weekend::vec3::{dot, unit_vector, write_color, Vec3};

fn ray_color(r: Ray) -> Vec3 {
    let t = hit_the_sphere(Vec3::new(0., 0., -1.), 0.5, &r);
    if t > 0.0 {
        let n = unit_vector(r.at(t) - Vec3::new(0., 0., -1.));
        return 0.5 * Vec3::new(n.x() + 1., n.y() + 1., n.z() + 1.);
    }
    let unit_direction = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn hit_the_sphere(center: Vec3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin() - center;
    let a = r.direction().length_squared();
    let half_b = dot(oc, r.direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / a;
    }
}

fn main() {
    // Image

    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 1080;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

    // Camera

    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f64 = 1.0;

    // Here I can't use const because my operators overloading are not const compatible
    // there might be a way to do it but I don't know it yet...
    let origin: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let horizontal: Vec3 = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner: Vec3 =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);
    let pb = ProgressBar::new(IMAGE_HEIGHT as u64);
    pb.set_style(
        ProgressStyle::with_template("[{elapsed_precise}] {bar:60.cyan/blue} {pos:>7}/{len:7}")
            .unwrap(),
    );
    let mut content = String::new();
    content.push_str("P3\n");
    content.push_str(&format!("{} {}\n", IMAGE_WIDTH, IMAGE_HEIGHT));
    content.push_str("255\n");
    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let u: f64 = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v: f64 = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(r);
            content.push_str(&get_color_str(pixel_color));
        }
        pb.inc(1);
    }
    pb.finish_with_message("done");
    std::fs::write("image.ppm", content).unwrap();
    eprintln!("");
}
