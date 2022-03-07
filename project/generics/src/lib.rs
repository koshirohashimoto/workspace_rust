use std::fmt::{Debug, Display};

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait Summary {
    fn summarize(&self) -> String;
}

pub fn some_func<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
    1
}

pub fn return_sammarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course."),
        reply: false,
        retweet: false,
    }
}

pub fn tweeter() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course."),
        reply: false,
        retweet: false,
    };
    
    println!("New tweet: {}", tweet.summarize());
}
