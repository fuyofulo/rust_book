fn main() {
    
    let statement = true;

    let number = if statement {5} else {6};

    println!("number is {}", number);

    let array = [1, 2, 3, 4, 5];

    for element in array {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}");
    }

    let mut index = 0;

    while index<5 {
        println!("the value of element at index {index} is {}", array[index]);
        index = index + 1;
    }

    println!("index printing will stop at 10");

    loop {
        println!("index value is {index}");
        index = index + 1;

        if index>10 {
            break;
        }
    }

}
