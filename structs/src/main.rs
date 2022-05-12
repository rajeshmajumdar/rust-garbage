struct User {
    email: String,
    name: String,
    active: bool,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        email: String::from("example@example.com"),
        name: String::from("example"),
        active: true,
        sign_in_count: 1
    };
    user1.email = String::from("anotheremail@example.com");

    let em = String::from("function@example.com");
    let na = String::from("functionname");
    let mut user2 = build_user(em, na);
    println!("Name: {}", user2.name);
}


fn build_user(email: String, name: String) -> User {
    User {
        email,
        name,
        active: true,
        sign_in_count: 1,
    }
}

// In the User struct we are using the String type instead of normal &str slice type
// because, if we want the struct to own the String, instead if we just pass the
// reference to the string slice type, we will encounter lifetime errors.
