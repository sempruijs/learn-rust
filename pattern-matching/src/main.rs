fn main() {
    let x = 200;

    match x {
        y @ 1..=7 => println!("{y} is between 1 and 7"),
        z if z < -3 => println!("{z} is lower than -3"),
        a @ (100 | 200 | 300) => println!("{a} is a big round number"),
        _ => println!("something else"),
    }
}
