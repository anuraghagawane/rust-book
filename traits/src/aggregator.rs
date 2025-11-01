use std::fmt::{Display, Formatter, Result};
pub trait Summary {
    fn summarize_author(&self) -> String { 
        String::from("ananomous")
    }
    fn summarize(&self) -> String {
        format!("Read more from {}", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} {} {}", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }
}

impl Display for SocialPost {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "username: {}, content: {}, reply: {}, repost: {}", self.username, self.content, self.reply, self.repost)
    }
}
