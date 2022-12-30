fn main() {
    let temperture = fahrenheit_to_celcius(-40.0);
    let number = get_fibonacci(4);
    print!("{temperture}\n");

    print!("{number}\n");

    show_lyrics();
}

fn fahrenheit_to_celcius(t: f64) -> f64 {
    ((t - 32.0) * 5.0) / 9.0
}

fn get_fibonacci(n: u32) -> u32 {
    if n == 0 || n == 1 {
        1
    } else {
        get_fibonacci(n - 2) + get_fibonacci(n - 1)
    }
}

fn show_lyrics() {
    let love_mail = [
        "A partridge in a pear tree",
        "Two turtle doves, and",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    let count = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eigth", "ninth",
        "tenth", "eleventh", "twelth",
    ];

    for i in 0..(love_mail.len()) {
        println!(
            "On the {} day of Christmas, my true love sent to me",
            count[i]
        );
        for j in (0..i).rev() {
            println!("{}", love_mail[j])
        }
        println!("\n")
    }
}
