use std::fmt;

use core::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl OutlinePrint for Point {
    fn outline_print(&self) {
        let input = self.to_string();
        let len = input.len();
        println!("{}", "*".repeat(len + 4));
        println!("* {} *", input);
        println!("{}", "*".repeat(len + 4));
    }
}

fn main() {
    let a = Point { x: 5, y: 5 };
    let b = Point { x: 10, y: 3 };

    let c = a + b;
    println!("{}", c);

    c.outline_print();
}

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let input = self.to_string();
        let len = input.len();
        println!("{}", "*".repeat(len + 4));
        println!("* {} *", input);
        println!("{}", "*".repeat(len + 4));
    }
}
