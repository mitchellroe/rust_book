fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    println!("user1.username: {}", user1.username);
    println!("user1.active: {}", user1.active);
    println!("user1.email: {}", user1.email);
    println!("user1.sign_in_count: {}", user1.sign_in_count);

    // println!("Taking over some of the fields from user1 to give to user2");
    let user2 = User {
        username: String::from("someOtherUser"),
        email: String::from("user2@example.com"),
        ..user1
    };
    println!("user2.username: {}", user2.username);
    println!("user2.active: {}", user2.active);
    println!("user2.email: {}", user2.email);
    println!("user2.sign_in_count: {}", user2.sign_in_count);

    println!("Now what happened to user1?");
    // println!("user1.username: {}", user1.username);
    println!("user1.active: {}", user1.active);
    // println!("user1.email: {}", user1.email);
    println!("user1.sign_in_count: {}", user1.sign_in_count);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        // // Instead of doing this...
        // username: username,
        // email: email,
        // // We can do this, which is the *field init shorthand*
        username,
        email,
        sign_in_count: 1,
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
