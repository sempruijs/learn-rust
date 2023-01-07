// use std::collections::HashMap;
// use std::collections;

fn main() {
    let x = median(vec![1, 2, -3, 4, 5]);
    println!("{}", x);
}

// median is the middle number in a list when sorted.
fn median(v: Vec<i32>) -> i32 {
    // if len is even, it will ceil it.
    let middel_index = v.len() / 2;
    v[middel_index]
}

// mod vector {
//     pub fn show_vector(v: &Vec<i32>) {
//         for i in v {
//             print!("{}\n", i);
//         }
//     }

//     pub fn add_fifty(v: &mut Vec<i32>) {
//         for i in v {
//             *i += 50;
//         }
//     }
// }
