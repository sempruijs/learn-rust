fn main() {
    println!("Hello, world!");

    let mut name = String::from("Sem");

    let x = get_length(&name);

    println!("{x}");
    print!("{}", name);

    add_sunshine(&mut name);
    print!("{}", name);
}

fn get_length(s: &String) -> usize {
    s.len()
}

fn add_sunshine(s: &mut String) {
    s.push_str("🌞")
}

fn test(bla: String) {
    let mut x = 5;
    println!("{}", x);
}
