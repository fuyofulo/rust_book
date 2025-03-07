use std::io::stdin as yolo;
fn main() {

    let mut s1 = String::from("");
    println!("Enter a sentence: ");

    yolo()
        .read_line(&mut s1)
        .expect("Failed to read the sentence");

    let word = get_first_word(&s1);
    println!("The first word is {}", word);
}

fn get_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

