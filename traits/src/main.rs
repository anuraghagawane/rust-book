use aggregator::{SocialPost, Summary};
use std::fmt::Display;

mod aggregator;

// fn notify<T: Summary + Display> (item: &T) -> String {
//     format!("Breaking new! {} {}", item.summarize(), item)
// }

fn notify<T>(item: &T) -> String
where
    T: Summary + Display
{
    format!("Breaking new! {}\n{}", item.summarize(), item)
}

fn notify1(item: &(impl Summary + Display)) -> String {
    format!("Breaking news! {}", item.summarize())
}

fn returns_summarizable () -> impl Summary + Display {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self{x, y}
    }
}


// conditional method implmentation based on trait bounds
impl<T: Display + PartialOrd> Pair<T> {
    fn cmd_display(&self) {
        if self.x >= self.y {
            println!("the largest member is x = {}", self.x);
        }
        else {
            println!("the largest member is y = {}", self.y);
        }
    }
}

// blanket implmentation
// we can implement a trait for any type that implments trait Display
// impl<T: Display> ToString for T {
//     // implementation goes here
// }

fn main() {
    let post = SocialPost {
        username: String::from("Horse_ebooks"),
        content: String::from("of course, as you probably"),
        reply: false,
        repost: false,
    };

    println!("1 new post: {}", post.summarize());
    println!("{}", notify(&post));
    notify1(&post);

    let post = returns_summarizable();
    println!("1 new post: {}", post.summarize());
    println!("{}", notify(&post));
    notify1(&post);

    let pair = Pair::new(1, 2);
    pair.cmd_display();

    // will not compile as post doesn't proper traits impled
    // let post1 = returns_summarizable();
    // let post2 = returns_summarizable();
    // let pair = Pair::new(post1, post2);
    // pair.cmd_display();
}
