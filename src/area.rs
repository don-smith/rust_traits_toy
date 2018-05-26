//! A container for the Area trait

/// Functions related to area
pub trait Area {
    fn get_area(&self) -> f64;
    fn is_larger_than(&self, &Self) -> bool;
}
