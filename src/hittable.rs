use crate::{vec3::*, ray::*};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Point3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            point: Point3::new(),
            normal: Point3::new(),
            //mat_ptr: Rc::new(Lambertian::from(color::new())),
            t: 0.,
            front_face: true,
        }
    }
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(r.direction(), outward_normal) < 0.;
        self.normal = if self.front_face {
            outward_normal
        } else {
            - outward_normal
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_rec: &mut HitRecord) -> bool;
}