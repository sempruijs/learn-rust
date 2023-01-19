use std::fmt::Display;

fn main() {
    let numbers = [1, 3, 4, 100, 3];
    let word = vec!['a', 'b', 'z', 'd'];

    let largest_from_numbers = largest(&numbers);
    let largest_from_word = largest(&word[..]);

    println!("{}", largest_from_numbers);
    println!("{}", largest_from_word);

    let point = Point::new(5, 6);
    println!("{:?}", point);

    let s1 = String::from("short");
    let s2 = String::from("very long string. long.");

    let result = longest_with_announcement(&s1, &s2, "here is the longest string!!!!");
    println!("{}", result);
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

fn longest_with_announcement<'a, T: Display>(s1: &'a str, s2: &'a str, announcement: T) -> &'a str {
    println!("announcement: {}", announcement);
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
