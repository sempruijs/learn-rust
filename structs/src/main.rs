fn main() {
    let user1 = build_user(String::from("harko@example.com"), String::from("harko"));
    let user2 = User {
        username: String::from("Clarla"),
        ..user1
    };

    let blue = Color(0, 0, 255);
    let origin = Point(0, 0, 0);

    println!("{}", blue.0);

    println!("{}", user1.username);
    println!("{}", user2.username);
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
