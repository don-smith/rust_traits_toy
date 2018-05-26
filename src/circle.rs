#![allow(unused_variables)] // because we don't use x and y anywhere

pub mod area;
pub mod circle_area;

/// A positional circle with a given size
pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
}
