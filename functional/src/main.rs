use std::thread;

fn main() {
    let v1 = vec![1, 4, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    for num in v2 {
        println!("{}", num);
    }

    let a = v1.iter().next().unwrap();
    let b = v1.iter().next().unwrap();

    println!("{}{}", a, b);
}
