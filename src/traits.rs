use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("Read more from {}...", self.summarize_author())
    }
}

pub struct NewsArticle {
    headline: String,
    author: String,
    content: String,
    location: String,
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summary for SocialPost {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_desugared<T: Summary>(item: &T) {
    notify(item);
}

pub fn notify_two(item1: &impl Summary, item2: &impl Summary) {
    notify(item1);
    notify(item2);
}

pub fn notify_two_same_type<T: Summary>(item1: &T, item2: &T) {
    notify(item1);
    notify(item2);
}

pub fn notify_with_multiple_trait_bounds(item: &(impl Summary + Display)) {
    println!("{item}");
    println!("{}", item.summarize());
}

// Long trait bounds
pub fn some_function1<T: Display + Clone, U: Display + Clone>(t: &T, u: &U) -> i32 {
    println!("{t}");
    println!("{u}");
    0
}

// Simplify with where clause
pub fn some_function2<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    println!("{t}");
    println!("{u:?}");
    0
}

// Returning types that implement traits
//
pub fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("kvkenyon"),
        content: String::from("This is a tweet"),
        reply: false,
        repost: false,
    }
}

// Using a trait bound with an impl block that uses generic type parameters,
// we can implement methods conditionally for types that implement the specified traits

#[derive(Debug)]
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x: {}", self.x);
        } else {
            println!("The largest member is y: {}", self.y);
        }
    }
}

pub fn test() {
    println!("[traits] Loading traits tests...");
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    };

    println!("1 new post: {}", post.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
    println!("Article: {}", article.content);

    notify(&article);
    notify(&post);

    let pair = dbg!(Pair::new(1.2, 3.2));

    println!("pair = ({}, {})", pair.x, pair.y);

    pair.cmp_display();

    println!("[traits] Tests done.");
}
