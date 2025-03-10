enum Color {
    Blue,
    Red,
    Green,
    Yellow
}

fn main() {
    
    check_color(Color::Blue);
    check_color(Color::Green);
    check_color(Color::Red);
    check_color(Color::Yellow);
    
}

fn check_color(c: Color) {
    match c {
        Color::Blue => println!("blue"),
        Color::Red => println!("red"),
        Color::Green => println!("green"),
        Color::Yellow => println!("yellow"),
    }
}
