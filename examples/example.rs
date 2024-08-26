use std::collections::HashMap;

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut users = HashMap::new();

    let new_user = User {
        username: String::from("johndoe"),
        email: String::from("john@example.com"),
        sign_in_count: 1,
        active: true,
    };

    users.insert(new_user.username.clone(), new_user);

    match users.get("johndoe") {
        Some(user) => println!("Found user: {} ({})", user.username, user.email),
        None => println!("User not found"),
    }

    for (username, user) in &users {
        println!(
            "User {} has signed in {} times",
            username, user.sign_in_count
        );
    }
}
