fn main() {
    
    let s = gives_ownership();
    println!("s: {s}");

    let s1 = String::from("yolo");
    
    let s3 = takes_and_returns_back(s1);

    println!("s3: {s3}");
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    println!("inside gives ownership function, makes a string and gives the ownership to s");
    return some_string;
}

fn takes_and_returns_back(s2: String) -> String {
    println!("inside takes & gives back function, it takes string s1 as an argument and stores it in s2. so now s2 has the ownership and s1 is dropped. it returns back s2 and we are taking its ownership to s3");
    return s2;
}
