fn main() {
    let v1 = vec![1, 2, 5, 2];
    let mut v1_iter = v1.iter();

    v1_iter.next();
    let a = v1_iter.next().unwrap();
    let b = v1_iter.next().unwrap();
    let c = v1_iter.next().unwrap();
    println!("{}{}{}", a, b, c);
    println!("{}", v1[0]);
}
