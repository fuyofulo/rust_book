
fn main() {
    match divide(10, 2) {
        Ok(result) => println!("10 divided by 2: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match divide(10, 0) {
        Ok(result) => println!("10 divided by 0 {}", result),
        Err(e) => println!("Error: {}", e)
    }
}

fn divide (a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a/b)
    }
}

