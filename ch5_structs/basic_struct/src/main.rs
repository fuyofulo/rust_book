#[derive(Debug)]
struct User {
    name: String,
    email: String,
    age: u32,
    country: String,
    disabled: bool
}

fn main() {

    let user1 = User {
        name: String::from("Fuyofulo"),
        email: String::from("fuyofulo@gmail.com"),
        age: 21,
        country: String::from("India"),
        disabled: false
    };

    println!("{:?}", user1);
}
