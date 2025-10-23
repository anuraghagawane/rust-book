fn main() {
    let s1 = String::from("hello world");
    let s = first_word(&s1);
    println!("{s}");
    let s = first_word(&s1[0..6]);
    println!("{s}");

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
