#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    fn value_in_cent(&self) -> u32 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}

fn main() {
    let coin = Coin::Nickel;

    let x: Option<i32> = None;

    if let Some(x) = x {
        println!("x is something, it\'s {}", x);
    } else {
        println!("It is nothing");
    }
    println!("{}", coin.value_in_cent());
}
