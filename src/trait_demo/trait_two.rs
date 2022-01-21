use std::fmt::{Debug, Display};

// Default implementations can call other methods in the same trait
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    // notify(tweet);

    notify_actually(tweet);
}

pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_actually<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

/// ...
/// # pub fn notify(item: impl Summary + Display) { }
/// # pub fn notify<T: Summary + Display>(item: T) { }
///
// specifying Multiple Trait Bounds with the + Syntax
// clearer Trait Bounds with where Clauses
/// # fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 { }
/// ...
fn some_function<T, U>(t: T, u: U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    1
}

// returning Types that Implement Traits
fn returns_summarized() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
