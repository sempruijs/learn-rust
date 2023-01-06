fn main() {
    let name = "sem";
    let c = name.chars().nth(1).unwrap();

    println!("{}", c)
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
