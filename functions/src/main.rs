fn main() {
    print_value(5, 10);
    print_five();
}

fn print_value(x: i32, y: i32) {
    println!("the value of x is {x} \n and the value of y is {y}");
}

fn five() -> i32 {
    5
}

fn print_five() {
    println!("there is {}", five());
}
