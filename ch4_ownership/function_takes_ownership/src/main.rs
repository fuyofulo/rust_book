fn main() {
    
    let s1 = String::from("hello");

    takes_ownership(s1); // ownership was s1 was given to s2 so s1 is dropped 


    // lol now we dont have both s1 and s2

    let x = 5;

    makes_copy(x);

    println!("x can still be used and its value is: {x}");
}

fn makes_copy(y: u32) {
    println!("value of x was copied in y\ny = {y}");
} // y is dropped 

fn takes_ownership(s2: String) {
    println!("value was s1 was not cloned but taken by s2\ns2 = {s2}");
} //s2 is dropped 


