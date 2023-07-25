use std::sync::Arc;

use crate::{
    camera::Camera,
    helper::{random, random_range},
    hittable_list::HittableList,
    material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal, Scatterable},
    object::Sphere,
    vec3::Vec3,
};

pub fn scene_default_camera(aspect_ratio: f64) -> (HittableList, Camera) {
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

pub fn scene_random(aspect_ratio: f64) -> (HittableList, Camera) {
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
