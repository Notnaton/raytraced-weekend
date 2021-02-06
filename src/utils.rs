use std::f64::MAX;
use rand::{thread_rng, Rng};

pub const infinity: f64 = f64::MAX;
const pi: f64 = 3.1415926535897932385;

pub fn degrees_to_radians(degrees: f64) -> f64{
    degrees * pi / 180.0
}

pub fn random_f64(min:f64, max:f64) -> f64 {
    thread_rng().gen_range(min, max)
}

pub fn clamp(x:f64, min:f64, max:f64) -> f64 {
    if x < min {return min}
    if x > max {return max}
    return x
}