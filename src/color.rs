#![allow(dead_code)]

use crate::{vec3::*, utils::clamp};

pub fn write_color(data: &mut Vec<u8> ,pixel_color: Color, samples_per_pixel: usize) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    let scale: f64 = 1.0 / samples_per_pixel as f64;
    
    r = (r * scale).sqrt();
    g = (g * scale).sqrt();
    b = (b * scale).sqrt();


    
    data.push((255.999 * clamp(r, 0.0, 0.999)) as u8);
    data.push((255.999 * clamp(g, 0.0, 0.999)) as u8);
    data.push((255.999 * clamp(b, 0.0, 0.999)) as u8);
}