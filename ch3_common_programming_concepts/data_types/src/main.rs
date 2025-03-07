fn main() {
    let x: u32 = 45;
    let y: f32 = 45.0;
    let z: i32 = -45;
    let c: char = 'c';
    let tuple: (i32, f32, u32) = (-1, 0.0, 1);
    let boolean: bool = true;
    let array = [1,2,3,4,5]; // array of numbers 
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    let array2: [i32; 5] = [6,7,8,9,10]; // array of i32 with 5 elements
    let array3 = [3; 5]; // array of 5 elements and every element is 3
    let august = months[7];

    println!("x: {}", x);
    println!("y: {}", y);
    println!("z: {}", z);
    println!("c: {}", c);
    println!("tuple: {:?}", tuple);
    println!("boolean: {}", boolean);
    println!("array: {:?}", array);
    println!("months: {:?}", months);
    println!("array2: {:?}", array2);
    println!("array3: {:?}", array3);
    println!("august: {}", august);
    
}
