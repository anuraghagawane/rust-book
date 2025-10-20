fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{}", guess);

    let x = 2.0;
    let y: f32 = 3.0;

    println!("{} {}", x, y);
    // let sum = 5 + 10;

    // subtraction
    // let difference = 95.5 - 4.3;

    // multiplication
    // let product = 4 * 30;

    // division
    // let quotient = 56.7 / 32.2;
    // let truncated = -5 / 3; // Results in -1

    // remainder
    // let remainder = 43 % 5;

    // tuples
    let tup:(u32, f64, bool) = (500, 6.4, true);

    println!("{} {} {}", tup.0, tup.1, tup.2);

    let (a, b, c) = tup;

    println!("{} {} {}", a, b, c);
    
    // Arrays
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", a);
    let a = [1, 2, 3, 4, 5];
    println!("{:?}", a);
    let a = [3; 5];
    println!("{:?}", a);

    println!("{}", a[0]);
    // println!("{}", a[10]); // panics

}
