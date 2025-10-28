fn main() {
    let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string();

    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");

    println!("{s}");

    let s1 = String::from("tic");
    let s2 = String::from("toe");

    let s3 = s1 + &s2;
    println!("{s3}");

    let s1 = String::from("tic");
    let s2 = String::from("toe");

    let s3 = format!("{s1}-{s2}");
    println!("{s3}");

    let s1 = String::from("hi");
    let h = &s1[0..];
    println!("{h}");

    let hello = "Здравствуйте";
    let answer = &hello[0..4];
    println!("{answer}");

    for c in "Здравствуйте".chars() {
        println!("{c}");
    }

    for c in "Здравствуйте".bytes() {
        println!("{c}");
    }
}
