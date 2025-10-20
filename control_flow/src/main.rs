fn main() {
    let number: i32 = 11;

    if number > 10 {
        println!("number is greater than 10");
    } else {
        println!("number is less than 10");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    // both types in the return blocks should be same
    let number = if condition {5} else {6};

    println!("The number is {number}");

}

