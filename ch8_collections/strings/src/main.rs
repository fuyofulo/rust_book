fn main() {
    // ways to create a string 
    // 1
    let mut s1 = String::new();
    //2
    let data = "initial contents";
    let s2 = data.to_string();
    println!("{s2}");
    //3
    let s3 = String::from("Hello");
    println!("{s3}");

    // updating a string
    //
    // appending a string
    let mut s4 = String::from("foo");
    s4.push_str(" bar");

    let s5 = " foo bar";
    s4.push_str(s5);

    // if you need to append a single character
    s4.push('!');

    println!("{s4}");


    let s6 = s4;
    println!("s6 took s4: {}", s6); //s4 is now gone, s6 is the new s4, we can also borrow reference if we want to keep both

    let s4 = &s6;
    println!("s4: {}, s6: {}", s4, s6);

    //concatenation
    let s7 = s2 + &s6;  // we cant borrow two string cuz then it needs ownership of atleast one string to exist. thats why I need to be without reference
                        // here s7 takes wonership of s2 and adds s6 to it as borrow
    println!("{s7}");

    // indexing
    let s8 = String::from("fuyofulo");
    let slice_of_s8_v1 = &s8[0..1];
    println!("{slice_of_s8_v1}");
    let slice_of_s8_v2 = s8.chars().nth(0);
    match slice_of_s8_v2 {
        Some(c) => println!("{c}"),
        None => println!("Empty string")
    }

    let first_byte_of_s8 = s8.bytes().nth(0);
    match first_byte_of_s8 {
        Some(b) => println!("{b}"),
        None => println!("Empty string")
    }








}
