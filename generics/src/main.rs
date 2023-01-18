fn main() {
    let numbers = [1, 3, 4, 100, 3];
    let word = vec!['a', 'b', 'z', 'd'];

    let largest_from_numbers = largest(&numbers);
    let largest_from_word = largest(&word[..]);

    println!("{}", largest_from_numbers);
    println!("{}", largest_from_word);
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
