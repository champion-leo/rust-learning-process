use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use ray_tracing_in_one_weekend::ray::Ray;
use ray_tracing_in_one_weekend::vec3::{dot, get_color_str, unit_vector, Vec3};

use std::sync::mpsc;
use std::thread;

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

struct ThreadResult {
    content: String,
    thread_index: u32,
}

fn main() {
    const THREADS: u32 = 8;
    let mut image_parts = (0..THREADS)
        .into_iter()
        .map(|_| String::new())
        .collect::<Vec<String>>();
    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];

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
    let m = MultiProgress::new();
    let style =
        ProgressStyle::with_template("[{elapsed_precise}] {bar:60.cyan/blue} {pos:>7}/{len:7}")
            .unwrap();

    let mut header = String::new();
    header.push_str("P3\n");
    header.push_str(&format!("{} {}\n", IMAGE_WIDTH, IMAGE_HEIGHT));
    header.push_str("255\n");
    for thread_index in 0..THREADS {
        let mut current_content = String::new();
        let min_height = thread_index * (IMAGE_HEIGHT / THREADS);
        let max_height = if thread_index == THREADS - 1 {
            IMAGE_HEIGHT
        } else {
            (thread_index + 1) * (IMAGE_HEIGHT / THREADS)
        };
        let tx = tx.clone();
        let pb = m.add(ProgressBar::new((max_height - min_height) as u64));
        pb.set_style(style.clone());
        handles.push(thread::spawn(move || {
            for j in (min_height..max_height).rev() {
                for i in 0..IMAGE_WIDTH {
                    let u: f64 = i as f64 / (IMAGE_WIDTH - 1) as f64;
                    let v: f64 = j as f64 / (IMAGE_HEIGHT - 1) as f64;
                    let r = Ray::new(
                        origin,
                        lower_left_corner + u * horizontal + v * vertical - origin,
                    );
                    let pixel_color = ray_color(r);
                    current_content.push_str(&get_color_str(pixel_color));
                }
                thread::sleep(std::time::Duration::from_millis(10));
                pb.inc(1);
            }
            pb.finish_with_message("done");
            tx.send(ThreadResult {
                content: current_content,
                thread_index,
            })
            .unwrap();
        }));
    }
    m.clear().unwrap();
    let mut body = String::new();
    for _ in handles {
        let part = rx.recv().unwrap();
        // MAYBE NOT OPTIMAL TO CLONE A LONG STRING LIKE THAT
        image_parts[(THREADS - part.thread_index - 1) as usize].push_str(&part.content);
    }
    for part in image_parts {
        body.push_str(&part);
    }
    let mut file_content = String::new();
    file_content.push_str(&header);
    file_content.push_str(&body);
    std::fs::write("image.ppm", file_content).unwrap();
    eprintln!("");
}
