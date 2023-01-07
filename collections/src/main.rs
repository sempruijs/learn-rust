use std::collections::HashMap;

fn main() {
    let numbers = [1, 2, 3, 9, 3, 3];
    let average_of_numbers = average(&numbers);
    let modus_of_numbers = modus(&numbers);
    let median_of_numbers = median(&numbers);
    println!("{:?}\n", numbers);
    println!("the modus is {}", modus_of_numbers);
    println!("the average is {}", average_of_numbers);
    println!("the median is {}", median_of_numbers);
}

// median is the middle number in a list when sorted.
fn median(v: &[i32]) -> i32 {
    // if len is even, it will ceil it.
    let middel_index = v.len() / 2;
    v[middel_index]
}

fn average(v: &[i32]) -> f32 {
    let mut total = 0;

    for i in v {
        total += i;
    }

    total as f32 / v.len() as f32
}

// modus is the most frequent number.
// TODO: Return a list when there are multiple winners
fn modus(numbers: &[i32]) -> i32 {
    let mut map = HashMap::new();

    for number in numbers {
        let count = map.entry(number).or_insert(0);
        *count += 1;
    }

    let mut most_commen_value = 0;

    for number in numbers {
        if &most_commen_value < &map.get(number).copied().unwrap() {
            *&mut most_commen_value = map.get(number).copied().unwrap();
        }
    }

    most_commen_value
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
