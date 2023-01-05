fn main() {
    let mut v = vec![1, 2, 3, 100];

    vector::show_vector(&v);

    vector::add_fifty(&mut v);

    vector::show_vector(&v);
}

mod vector {
    pub fn show_vector(v: &Vec<i32>) {
        for i in v {
            print!("{}\n", i);
        }
    }

    pub fn add_fifty(v: &mut Vec<i32>) {
        for i in v {
            *i += 50;
        }
    }
}
