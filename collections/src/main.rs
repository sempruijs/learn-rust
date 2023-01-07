// use std::collections::HashMap;
// use std::collections;

fn main() {
    let numbers = [1, 2, 3, 9];
    let modes_of_numbers = modus(&numbers);
    println!("{}", modes_of_numbers);
}

// median is the middle number in a list when sorted.
fn median(v: &[i32]) -> i32 {
    // if len is even, it will ceil it.
    let middel_index = v.len() / 2;
    v[middel_index]
}

fn modus(v: &[i32]) -> f32 {
    let mut total = 0;

    for i in v {
        total += i;
    }

    total as f32 / v.len() as f32
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
