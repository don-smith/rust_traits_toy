//! The implementation for the area of a circle

use Circle;
use area::Area;
use std::f64::consts::PI;

impl Area for Circle {
    /// Returns the area of the Circle.
    ///
    /// ```
    /// use circle::area::Area;
    /// let circle = circle::Circle { x: 2.0, y: 3.0, radius: 1.4 };
    /// assert_eq!(circle.get_area(), 6.157521601035993);
    /// ```
    fn get_area(&self) -> f64 {
        PI * (self.radius * self.radius)
    }

    /// Returns true if the current circle is larger than the circle provided.
    ///
    /// ```
    /// use circle::area::Area;
    /// let c1 = circle::Circle { x: 2.0, y: 3.0, radius: 1.4 };
    /// let c2 = circle::Circle { x: 2.0, y: 3.0, radius: 1.5 };
    /// assert!(c2.is_larger_than(&c1));
    /// ```
    fn is_larger_than(&self, other: &Self) -> bool {
        self.get_area() > other.get_area()
    }
}
