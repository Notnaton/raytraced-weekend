use crate::{ray::*, hittable::*, vec3::*};

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self {
            center: center, radius: radius
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction().lenght_sqared();
        let b = Vec3::dot(oc, ray.direction());
        let c = oc.lenght_sqared() - self.radius * self.radius;
        
        let descriminant = (b*b) - (a*c);
        
        if descriminant < 0. {return false}

        let sqrtd = descriminant.sqrt();

        let mut root = (- b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.point = ray.at(rec.t);
        let outward_normal = (rec.point - self.center) / self.radius;
        rec.set_face_normal(ray, outward_normal);

        
        true
    }
}