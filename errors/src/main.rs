use std::fs::{self, File};
use std::io::{self, Read};

fn main() {
    let username_result = read_username_from_filename(String::from("hello.txt"));
    match username_result {
        Ok(s) => println!("{}", s),
        Err(error) => panic!("Error recieving username: {:?}", error),
    };

    let string_result = fs::read_to_string("bye.txt");
    let last_char = match last_char_of_first_line(&string_result.unwrap()) {
        Some(c) => c,
        None => ' ',
    };

    println!("{}", last_char);
}

fn read_username_from_filename(filename: String) -> Result<String, io::Error> {
    let mut username = String::new();
    File::open(filename)?.read_to_string(&mut username)?;

    Ok(username)
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
