#[derive(Debug)]
struct Rectangle {
    x: u32,
    y: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.x > other.x && self.y > other.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { x: 5, y: 5 };
        let smaller = Rectangle { x: 3, y: 3 };

        assert!(larger.can_hold(&smaller));
    }
}
