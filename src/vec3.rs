#![allow(dead_code)]

use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Deref, DerefMut, Neg};
use crate::{utils::*};

pub type Point3 = Vec3; 
pub type Color = Vec3;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3 ([f64; 3]);

impl Vec3 {
    pub fn new() -> Self {
        Self ([0., 0., 0.])
    }
    pub fn from(e0: f64, e1: f64, e2: f64) -> Self {
        Self ([e0, e1, e2])
    }
    
    pub fn x(&self) -> f64 {self[0]}
    pub fn y(&self) -> f64 {self[1]}
    pub fn z(&self) -> f64 {self[2]}
    // pub fn x_mut(&mut self) -> &mut f64 {&mut self[0]}
    // pub fn y_mut(&mut self) -> &mut f64 {&mut self[1]}
    // pub fn z_mut(&mut self) -> &mut f64 {&mut self[2]}

    pub fn random(min:f64,max:f64) -> Self {
        Self([random_f64(min, max),random_f64(min, max),random_f64(min, max)])
    }
    pub fn random_in_unit_sphere(min:f64,max:f64) -> Self {
        loop {
            let p = Vec3::random(min, max);
            if p.lenght_sqared() >= 1. {continue}
            return p;
        }
    }
    pub fn random_unit_vector(min:f64,max:f64) -> Self {
        Vec3::random_in_unit_sphere(min, max).unit_vector()
    }
    pub fn random_in_hemisphere(normal: Vec3, min:f64,max:f64) -> Self {
        let in_unit_sphere = Vec3::random_in_unit_sphere(min, max);
        if Vec3::dot(in_unit_sphere, normal) > 0.0 {
            return in_unit_sphere
        }else{
            return - in_unit_sphere
        }
    }

    pub fn lenght_sqared(&self) -> f64{
        self[0]*self[0] + 
        self[1]*self[1] +  
        self[2]*self[2]
    }
    pub fn length(&self) -> f64 {
        self.lenght_sqared().sqrt()
    }

    pub fn dot(self, other: Self) -> f64 {
        self[0] * other[0] +
        self[1] * other[1] +
        self[2] * other[2]
    }
    pub fn cross(self, other: Self) -> Self {
        Vec3::from(
            self[1] * other[2] - self[2] * other[1], 
            self[2] * other[0] - self[0] * other[2], 
            self[0] * other[1] - self[1] * other[0]
        )
        
    }
    pub fn unit_vector(self) -> Self {
        self / self.length()
    }
}

impl Add<Self> for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self{
        Vec3::from(self[0] + other[0], self[1] + other[1], self[2] + other[2])
    }
}
impl Add<f64> for Vec3 {
    type Output = Self;
    fn add(self, other: f64) -> Self{
        Vec3::from(self[0] + other, self[1] + other, self[2] + other)
    }
}
impl AddAssign<Self> for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self[0] += other[0];
        self[1] += other[1];
        self[2] += other[2];
    }
}
impl AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, other: f64) {
        self[0] += other;
        self[1] += other;
        self[2] += other;
    }
}

impl Sub<Self> for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self{
        Vec3::from(self[0] - other[0], self[1] - other[1], self[2] - other[2])
    }
}
impl Sub<f64> for Vec3 {
    type Output = Self;
    fn sub(self, other: f64) -> Self{
        Vec3::from(self[0] - other, self[1] - other, self[2] - other)
    }
}
impl SubAssign<Self> for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self[0] -= other[0];
        self[1] -= other[1];
        self[2] -= other[2];
    }
}
impl SubAssign<f64> for Vec3 {
    fn sub_assign(&mut self, other: f64) {
        self[0] -= other;
        self[1] -= other;
        self[2] -= other;
    }
}

impl Mul<Self> for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Vec3::from(self[0] * other[0], self[1] * other[1], self[2] * other[2])
    }
}
impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, other: f64) -> Self {
        Vec3::from(self[0] * other, self[1] * other, self[2] * other)
    }
}
impl MulAssign<Self> for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        self[0] *= other[0];
        self[1] *= other[1];
        self[2] *= other[2];
    }
}
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        self[0] *= other;
        self[1] *= other;
        self[2] *= other;
    }
}

impl Div<Self> for Vec3 {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Vec3::from(self[0] / other[0], self[1] / other[1], self[2] / other[2])
    }
}
impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, other: f64) -> Self {
        Vec3::from(self[0] / other, self[1] / other, self[2] / other)
    }
}
impl DivAssign<Self> for Vec3 {
    fn div_assign(&mut self, other: Self) {
        self[0] /= other[0];
        self[1] /= other[1];
        self[2] /= other[2];
    }
}
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        self[0] /= other;
        self[1] /= other;
        self[2] /= other;
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec3::from( -self[0], -self[1], -self[2])
    }
}

impl Deref for Vec3 {
    type Target = [f64; 3];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Vec3 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


