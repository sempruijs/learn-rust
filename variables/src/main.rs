fn main() {
    let x = 5;

    println!("x = {x}");

    let x = 6;

    {
        let x = x * 2;
        println!("{x}");
    }
    println!("x = {x}");
}
