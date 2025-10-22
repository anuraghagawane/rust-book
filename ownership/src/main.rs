fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    //if we don't clone s1 then s1 will be invalid
    println!("{s1}, world");
    println!("{s2}, world");

    let mut s = String::from("hello");
    s = String::from("ahoy");
    let mut s = take_ownership(s);

    let x = 5;
    make_copy(5);


    // REFERENCES AND BORROWING

    let len = cal_len(&s);

    println!("length of s is {len}");

    println!("{s}, world!");

    // if we pass a mutable reference we can change the strings content
    change(&mut s); 

    println!("{s}");

    // cannot have more than one reference if we have a mutable reference
    // let r1 = &s;
    // let r2 = &mut s;

    // println!("{r1} {r2}");

    // let s = dangle();
    let s = no_dangle();
}

fn take_ownership(some_string: String) -> String {
    println!("{some_string}");
    some_string
}

fn make_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn cal_len(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}

// here the s goes out of scope and we return reference to that string which is a invalid string
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s;
// }

// we directly return the ownership of s
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
