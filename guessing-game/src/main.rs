use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to the guessing game.\n \n The secret number is between 1 and 100");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Enter your guess:");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to recieve input.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a number please:");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{guess} is too low"),
            Ordering::Greater => println!("{guess} is too high"),
            Ordering::Equal => {
                println!("That's right!");
                break;
            }
        }
        println!("Try again")
    }
}
