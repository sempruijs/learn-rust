pub struct Guess {
    value: u32,
}

impl Guess {
    fn new(x: i32) -> Guess {
        if x < 0 {
            panic!("guess should be equal or above 0. got {}", x);
        } else if x > 100 {
            panic!("guess should be equal or below 100. Got {}", x);
        }

        Guess { value: x as u32 }
    }
}

fn add_two(a: i32) -> i32 {
    a + 2
}

fn print_value_return_10(v: i32) -> u8 {
    println!("the value is {}", v);
    10
}

fn salut_name(n: &str) -> String {
    format!("salut {}", n)
}

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

    #[test]
    fn two_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn three_adds_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn salut_coreect() {
        let result = salut_name("harko");
        assert!(result.contains("salut harko"), "result was {}", result);
    }

    #[test]
    #[should_panic(expected = "equal or below 100")]
    fn greater_than_100() {
        Guess::new(101);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("2 + 2 doesn't match 4"))
        }
    }

    #[test]
    fn pass_value_print() {
        let value = print_value_return_10(4);
        assert_eq!(10, value);
    }

    #[test]
    #[ignore]
    fn fail_value_print() {
        let value = print_value_return_10(4);
        assert_eq!(5, value);
    }
}
