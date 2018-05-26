use std;
use area::Area;
use circle::Circle;

impl Area for Circle {
    fn get_area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }

    fn is_larger_than(&self, other: &Self) -> bool {
        self.get_area() > other.get_area()
    }
}
