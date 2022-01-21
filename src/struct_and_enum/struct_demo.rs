struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user: User = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("{}", user.email);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,    // email
        username, // username
        active: true,
        sign_in_count: 1,
    }
}
