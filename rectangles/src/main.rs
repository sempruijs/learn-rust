#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rec: &Rectangle) -> bool {
        self.width > rec.width && self.height > rec.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rec1 = Rectangle {
        width: 30,
        height: 20,
    };

    let rec2 = Rectangle {
        width: 10,
        height: 5,
    };

    let rec3 = Rectangle::square(10);

    print!("{}", rec2.can_hold(&rec1));
    println!("{}", rec1.can_hold(&rec3));
    println!("{}", rec1.area());
}
