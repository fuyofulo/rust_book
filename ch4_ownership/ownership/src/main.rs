fn main() {


    let x: i32 = 6;
    let y = x;

    println!("x: {x} & y: {y}");

    // this works for numbers or more precisely things that are stored in stack as they dont change
    // their size
    
    // for data that can change its size during runtime, we use heap to allocate memory and store
    // the data there. we only store the pointer of that heap location in the stack.

    let s1 = String::from("hello");
    let s2 = s1;

    // now we cant use s1. the pointer of s1 which was pointing towards the hello located in heap
    // is given to s2 and dropped from s1. so s1 is no longer defined and s2 id the new s1 

    println!("{}", s2);


    // if we want to use both s1 and s2 then we can use clone(). it makes a deep copy of the data
    // stored in heap.
    
    let s1 = s2.clone();

    println!("s1: {} & s2: {}", s1, s2);




}
