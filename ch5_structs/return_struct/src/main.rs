#[derive(Debug)]
struct User {
    name: String,
    password: String
}

fn main() {
 
    let u1 = get_struct(String::from("zaid"), String::from("password"));
    println!("{:?}", u1);

}

fn get_struct(n: String, p: String) -> User {

    let user = User {
        name: n,
        password: p
    };

    return user;
}
