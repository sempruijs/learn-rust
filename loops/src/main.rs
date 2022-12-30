fn main() {
    let temperture = fahrenheit_to_celcius(-40.0);
    let number = get_fibonacci(4);
    print!("{temperture}\n");

    print!("{number}\n");
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
