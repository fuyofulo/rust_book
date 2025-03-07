fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("s: {s}");
}

fn change(s2: &mut String) {
    s2.push_str(", world");
}
