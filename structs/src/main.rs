struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct AlwaysEqual;

fn main() {
    let mut user1 = build_user(String::from("anurag"), String::from("anurag"));

    user1.username = String::from("ah");

    let user2 = User {
        email: String::from("ah"),
        ..user1
    };

    println!("{}", user1.email);
    println!("{}", user2.username);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{}", black.0);

    let Point(x, y, z) = origin;
    
    let subject = AlwaysEqua;
}

fn build_user(email: String, username: String) -> User {
    let user1 = User {
        active: true,
        username,
        email,
        sign_in_count: 1
    };

    user1
}
