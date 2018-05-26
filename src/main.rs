mod area;
mod circle;
mod circle_area;

use circle::Circle;
use area::Area;

fn main() {
    let c1 = Circle { x: 7.9, y: 4.6, radius: 4.9 };
    let c2 = Circle { x: 6.3, y: 9.1, radius: 5.1 };
    println!("c1 area: {}", c1.get_area());
    println!("c2 area: {}", c2.get_area());
    println!("c1 is larger than c2: {}", c1.is_larger_than(&c2));
}
