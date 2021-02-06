#![allow(dead_code)]

use crate::vec3::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self{
        Ray {
            origin: origin,
            direction: direction
        }
    }

    pub fn origin(self) -> Point3 {
        self.origin
    }
    pub fn direction(self) -> Vec3 {
        self.direction
    }

    pub fn at(self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
}