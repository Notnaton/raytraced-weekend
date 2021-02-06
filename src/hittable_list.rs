use crate::{hittable::*, ray::*};
use std::rc::Rc;

pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new () -> Self {
        Self { objects: vec![] }
    }
    pub fn add (&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object)
    }
    pub fn clear (&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool{
        let mut temp_rec: HitRecord = HitRecord::new();
        let mut hit = false;
        let mut closest = t_max;

        for object in &self.objects {
            if object.hit(ray, t_min, closest, &mut temp_rec) {
                hit = true;
                closest = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
        hit
    }
}