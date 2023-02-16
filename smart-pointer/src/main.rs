use crate::List::{Cons, Nil};
use std::ops::Deref;
use std::rc::Rc;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("out of scope. Had data: {}", self.data);
    }
}

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn print_name(name: &String) {
    println!("hello, {}", name);
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let n = MyBox(String::from("Harko"));
    print_name(&n);

    let a = CustomSmartPointer {
        data: String::from("1"),
    };

    let b = CustomSmartPointer {
        data: String::from("2"),
    };

    let c = CustomSmartPointer {
        data: String::from("3"),
    };

    println!("created Custom Smart pointers");

    drop(b);

    let m = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("a has {} references", Rc::strong_count(&m));

    let n = Cons(2, Rc::clone(&m));

    println!("a has {} references", Rc::strong_count(&m));
    {
        let o = Cons(4, Rc::clone(&m));

        println!("a has {} references", Rc::strong_count(&m));
    }

    println!("a has {} references", Rc::strong_count(&m));

    println!("a has {} references", Rc::strong_count(&m));
}
