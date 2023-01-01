fn main() {
    let hello = String::from("Hello world");
    let word = first_word(&hello);

    let numbers = [2, 3, 1, 2, 2, 4];
    let slice = &numbers[2..4];

    println!("{:?}", slice);

    println!("{}", word);
}

fn first_word(s: &str) -> &str {
    let a = s.as_bytes();

    for (i, &char) in a.iter().enumerate() {
        if char == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
