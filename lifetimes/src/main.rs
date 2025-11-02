struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> & str {
        println!("Attention please: {announcement}");
        self.part
    }
}

fn main() {
    // this will not work as x does not live long enough and is deallocated before x uses it 
    // leading to a dangling pointer
    // let r;
    //
    // {
    //     let x = 5;
    //     r = &x;
    // }
    //
    // println!("r: {r}");

    let x = 5;
    let  r = &x;

    println!("r: {r}");

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("longest: {}", result);

    let string1 = String::from("loooooooooooooong");

    {
        let string2 = String::from("short");
        let result = longest(string1.as_str(), string2.as_str());
        println!("longest: {}", result);
    }

    // let string1 = String::from("loooooooooooooong");
    // let result;
    // {
    //     let string2 = String::from("short");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("longest: {}", result);

    let novel = String::from("call. some years ago");
    let first_sentence = novel.split(".").next().unwrap();

    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("{} {}", i.level(), i.announce_and_return_part("hello"));

    let s: &'static str = "I Have a static lifetime";
}

// lifetime here specifies => the returned reference will be valid as long as both parameters are
// valid
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
