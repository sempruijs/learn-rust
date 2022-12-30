fn main() {
    let s1 = String::from("Hello");
    let mut x = 4;

    show_string(s1);

    x = add_five(x);
    println!("{}", x);
    // println!("{s1}");
}

fn add_five(x: u32) -> u32 {
    x + 5
}

fn show_string(str: String) {
    println!("{}", str);
}
