fn main() {
    let temperture = fahrenheit_to_celcius(-40.0);
    print!("{}", temperture);
}

fn fahrenheit_to_celcius(t: f64) -> f64 {
    ((t - 32.0) * 5.0) / 9.0
}
