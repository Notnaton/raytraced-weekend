mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod utils;
mod camera;

use crate::{vec3::*, color::*, ray::*, hittable::*, utils::*, hittable_list::*, sphere::*, camera::*};

use std::path::Path;
use std::io::Write;
use std::fs::File;
use std::rc::Rc;
/*
pub fn hit_sphere(center: &Point3, radius: &f64, r: &Ray) -> f64 {
    let oc = r.origin - *center;
    let a = r.direction.lenght_sqared();
    let b = Vec3::dot(oc, r.direction);
    let c = oc.lenght_sqared() - radius*radius;
    let descriminant = (b*b) - (a*c);
    if descriminant < 0.{
        return -1.0;
    }else {
        return -b - (descriminant.sqrt()/a);
    }
}*/

pub fn ray_color(ray: Ray, world: &impl Hittable, depth: usize) -> Color {
    let mut rec = HitRecord::new();

    if depth <= 0 {
        return Color::new();
    }

    if world.hit(&ray, 0.001, infinity, &mut rec){
        //let target: Point3 = rec.point + rec.normal + Vec3::random_unit_vector(-1.,1.);
        let target: Point3 = rec.point + rec.normal + Vec3::random_in_unit_sphere(-1.,1.);
        //let target: Point3 = rec.point + Vec3::random_in_hemisphere(rec.normal, -1., 1.);
        return ray_color(Ray::new(rec.point, target - rec.point), world, depth -1) * 0.5;
        //return ( rec.normal + Color::from(1., 1., 1.)) * 0.5;
    }
    let unit_direction: Vec3 = Vec3::unit_vector(ray.direction);
    let t = 0.5* (unit_direction.y() + 1.0);
    Color::from(1.0, 1.0, 1.0)* (1.0-t) + Color::from(0.5, 0.7, 1.0) * t   
}

fn main() {
    // Image
    let aspect_ration: f64 = 16./9.;
    let image_width: usize = 1920;
    let image_height: usize = (image_width as f64 / aspect_ration) as usize;
    let samples_per_pixel = 20;
    let max_depth: usize = 20;

    let mut data: Vec<u8> = vec![];

    //World
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Point3::from(0.,0.,-1.), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::from(0.,-100.5,-1.), 100.)));

    // Camera
    let cam = Camera::new();

    //Render
    for i in (0..image_height).rev() {
        println!("Progress: {} of {} lines left", i, image_height);
        
        for j in 0..image_width {
            let mut pixel_color = Color::from(0.0, 0.0, 0.0);
            for _s in 0..samples_per_pixel {
                let u = (j as f64 + random_f64(0.,1.))/ (image_width-1) as f64;
                let v = (i as f64 + random_f64(0.,1.))/ (image_height-1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(r, &world, max_depth);
            }
            write_color(&mut data, pixel_color, samples_per_pixel);
        }
    }

    //Write to file
    let path = Path::new("/dev/rust/raytraced-weekend/output.ppm");
    let mut file = File::create(&path).unwrap();
    let header = format!("P6 {} {} 255\n", image_width, image_height);
    file.write(header.as_bytes()).unwrap();
    file.write(&data).unwrap();
}
