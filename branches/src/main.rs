fn main() {
    let number = 100;
    println!("The number is {number}");

    if number < 3 {
        println!("not so large");
    } else if number < 10 {
        println!("pretty large");
    } else {
        println!("Very large!")
    }

    print!("{}", if number < 4 { "whoa" } else { "blob" })
}
