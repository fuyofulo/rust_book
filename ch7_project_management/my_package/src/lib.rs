pub mod greetings {
    pub fn say_hello() {
        println!("Hello from the greetings module!");
    }
}

pub fn greet() {
    crate::greetings::say_hello();
}

// lib.rs doesnt need to have a main function as it only contains functions and the code doesnt
// start to execute from here. this is also called a crate. 
