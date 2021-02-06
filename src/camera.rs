use crate::{ray::*, vec3::*};

pub struct Camera {
    origin:             Point3,
    horizontal:         Vec3,
    vertical:           Vec3,
    lower_left_corner:  Vec3,
    aspect_ration:      f64,
    viewport_height:    f64,
    viewport_width:     f64,
    focal_length:       f64, 
}

impl Camera {
    pub fn new() -> Self{
        let aspect_ration: f64 = 16.0/9.0;
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = viewport_height * aspect_ration;
        let focal_length: f64 = 1.0;
        let origin = Point3::from(0.,0.,0.);
        let horizontal = Vec3::from(viewport_width, 0., 0.);
        let vertical = Vec3::from(0., viewport_height, 0.);
        let lower_left_corner = origin - horizontal /2. - vertical/2. - Vec3::from(0., 0., focal_length);

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            aspect_ration,
            viewport_height,
            viewport_width,
            focal_length,
        }
    }
    pub fn get_ray(&self, u: f64, v: f64) -> Ray{
        Ray::new(self.origin, self.lower_left_corner + self.horizontal*u + self.vertical*v)
    }
}