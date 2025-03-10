fn main() {
    let v = vec![1,2,3];
    v[50]; //this line will cause panic, rust also gives an explicit panic macro that we can use 
           // panic!("enter panic message")
           // panic is an unrecoverable error
}

