use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use ray_tracing_in_one_weekend::camera::{self, Camera};
use ray_tracing_in_one_weekend::helper::{random, random_range, INFINITY};
use ray_tracing_in_one_weekend::hittable_list::HittableList;
use ray_tracing_in_one_weekend::material::{self, Dielectric, Lambertian, Metal, Scatterable};
use ray_tracing_in_one_weekend::object::{Hittable, Sphere};
use ray_tracing_in_one_weekend::ray::Ray;
use ray_tracing_in_one_weekend::vec3::{get_color_str, unit_vector, Vec3};

use std::sync::{mpsc, Arc};
use std::thread;

fn ray_color(r: Ray, world: &HittableList, depht: i32) -> Vec3 {
    let hit_record = world.hit(&r, 0.001, INFINITY);
    if depht <= 0 {
        return Vec3::new(0., 0., 0.);
    }
    if hit_record.is_some() {
        let hit_record = hit_record.unwrap();
        let mut attenuation = Vec3::new(0., 0., 0.);
        let mut scattered = Ray::new(Vec3::new(0., 0., 0.), Vec3::new(0., 0., 0.));
        if hit_record
            .material
            .scatter(&r, &hit_record, &mut attenuation, &mut scattered)
        {
            return attenuation.clone() * ray_color(scattered, world, depht - 1);
        }
    }
    let unit_direction = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

struct ThreadResult {
    content: String,
    thread_index: u32,
}

fn scene_default_camera(aspect_ratio: f64) -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let material_ground = Arc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_right = Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0., -100.5, -1.),
        100.,
        material_ground,
    )));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0., 0., -1.),
        0.5,
        material_center,
    )));
    world.add(Arc::new(Sphere::new(
        Vec3::new(-1., 0., -1.),
        0.5,
        material_left.clone(),
    )));
    world.add(Arc::new(Sphere::new(
        Vec3::new(-1., 0., -1.),
        -0.45,
        material_left,
    )));
    world.add(Arc::new(Sphere::new(
        Vec3::new(1., 0., -1.),
        0.5,
        material_right,
    )));
    let lookfrom = Vec3::new(3., 3., 2.);
    let lookat = Vec3::new(0., 0., -1.);
    let vup = Vec3::new(0., 1., 0.);
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 2.0;

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );
    return (world, camera);
}

fn scene_random(aspect_ratio: f64) -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0., -1000., 0.),
        1000.,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random();
            let center = Vec3::new(a as f64 + 0.9 * random(), 0.2, b as f64 + 0.9 * random());

            if (center - Vec3::new(4., 0.2, 0.)).length() > 0.9 {
                let sphere_material: Arc<dyn Scatterable>;
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vec3::random() * Vec3::random();
                    sphere_material = Arc::new(Lambertian::new(albedo));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::random_in_range(0.5, 1.);
                    let fuzz = random_range(0., 0.5);
                    sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }

        let material1 = Arc::new(Dielectric::new(1.5));
        world.add(Arc::new(Sphere::new(Vec3::new(0., 1., 0.), 1., material1)));
        let material2 = Arc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
        world.add(Arc::new(Sphere::new(Vec3::new(-4., 1., 0.), 1., material2)));
        let material3 = Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.));
        world.add(Arc::new(Sphere::new(Vec3::new(4., 1., 0.), 1., material3)));
    }
    let camera = Camera::new(
        Vec3::new(13., 2., 3.),
        Vec3::new(0., 0., 0.),
        Vec3::new(0., 1., 0.),
        20.,
        aspect_ratio,
        0.1,
        10.,
    );
    return (world, camera);
}

// fn scene_wide_angle_camera(aspect_ratio: f64) -> (HittableList, Camera) {
//     let r: f64 = (std::f64::consts::PI / 4.0).cos();
//     let mut world = HittableList::new();

//     let material_left = Arc::new(Lambertian::new(Vec3::new(1., 0., 0.)));
//     let material_right = Arc::new(Lambertian::new(Vec3::new(0., 0., 1.)));

//     world.add(Arc::new(Sphere::new(
//         Vec3::new(-r, 0., -1.),
//         r,
//         material_left,
//     )));
//     world.add(Arc::new(Sphere::new(
//         Vec3::new(r, 0., -1.),
//         r,
//         material_right,
//     )));
//     let camera = Camera::new(
//         Vec3::new(-2., 2., 1.),
//         Vec3::new(0., 0., 0.),
//         Vec3::new(0., 0., -1.),
//         90.,
//         aspect_ratio,
//     );
//     return (world, camera);
// }

fn main() {
    const THREADS: u32 = 8;
    let mut image_parts = (0..THREADS)
        .into_iter()
        .map(|_| String::new())
        .collect::<Vec<String>>();
    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];

    // Image
    // TODO: remove duplicate ASPECT_RATIO here and in the camera
    const ASPECT_RATIO: f64 = 3. / 2.;

    const IMAGE_WIDTH: u32 = 1200;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 500;
    const MAX_DEPHT: i32 = 50;

    // World

    let (world, camera) = scene_random(ASPECT_RATIO);

    let world = Arc::new(world.clone());

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
        let world = world.clone();
        let camera = camera.clone();
        handles.push(thread::spawn(move || {
            for j in (min_height..max_height).rev() {
                for i in 0..IMAGE_WIDTH {
                    let mut pixel_color = Vec3::new(0., 0., 0.);
                    for _ in 0..=SAMPLES_PER_PIXEL {
                        let u: f64 = (i as f64 + random()) / (IMAGE_WIDTH - 1) as f64;
                        let v: f64 = (j as f64 + random()) / (IMAGE_HEIGHT - 1) as f64;
                        let r = camera.get_ray(u, v);
                        pixel_color += ray_color(r, &*world, MAX_DEPHT);
                    }
                    current_content.push_str(&get_color_str(pixel_color, SAMPLES_PER_PIXEL));
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
