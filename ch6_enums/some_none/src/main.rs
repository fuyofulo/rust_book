fn main() {
    let five = Some(5);
    let _none = plus_one(None);
    let _six = plus_one(five);
}

fn plus_one(x: Option<i32>) {
    match x {
        None => println!("None"),
        Some(i) => println!("{}", i+1),
    }
}

