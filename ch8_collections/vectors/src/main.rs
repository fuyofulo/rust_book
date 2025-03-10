fn main() {
    
    let mut v1 = Vec::new();
    let mut v2 = vec![8,7,6,5];

    v1.push(1);
    v1.push(2);
    v1.push(3);
    v1.push(4);

    v2.push(4);
    v2.push(3);
    v2.push(2);
    v2.push(1);

    println!("v1: {:?}, v2: {:?}", v1, v2);

    let first_of_v1 = &v1[0];
    println!("First element of v1: {}", first_of_v1);

    let first_of_v2 = v2.get(0);
    match first_of_v2 {
        Some(s) => println!("First element of v2: {}", s),
        None => println!("First element of v2 is empty")
    }

    let mut v3 = vec![0,1,2,3];
    let first = &v3[0];
    // v3.push(6); this line of code will not compile because we are using the refernce from the
    // above line in the print statement below.
    println!("The first element of v3: {}", first);


    // iterating over values of vector
    
    for i in &v2 {
        println!("{}", i);
    };

    for i in &mut v2 {
        *i = &*i*10;
        println!("{}", i);
    }

    for i in v2 {
        println!("{}", i);
    }

}
