use std::thread;

fn main() {
    let square = |x| x * x;
    let five_times_five = square(5);
    println!("{}", five_times_five);

    let mut list = vec![1, 3, 2];
    println!("{:?}", list);
    let mut borrow_mut = || list.push(7);
    borrow_mut();
    println!("{:?}", list);

    thread::spawn(move || println!("from thread: {:?}", list))
        .join()
        .unwrap();
}
