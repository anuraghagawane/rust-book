use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10); 
    scores.insert(String::from("Yellow"), 50); 

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{score}");

    for (key, value) in &scores{
        println!("{key}: {value}");
    }


    let mut map = HashMap::new();
    let field_name = String::from("color");
    let field_value = String::from("Blue");
    map.insert(field_name, field_value);
    println!("{map:?}");

    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("blue"), 20);

    println!("{scores:?}");

    scores.entry(String::from("yellow")).or_insert(30);
    scores.entry(String::from("blue")).or_insert(50);

    println!("{scores:?}");

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}")
}
