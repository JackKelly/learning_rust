struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        email: String::from("foo@bar.com"),
        username: String::from("foobar"),
        active: true,
        sign_in_count: 1,
    };

    println!("{}", user1.email);

    let user2 = build_user(String::from("blah"), String::from("foo"));

    println!("{}", user2.email);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}