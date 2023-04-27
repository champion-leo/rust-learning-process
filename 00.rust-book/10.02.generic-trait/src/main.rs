use std::fmt::{Display, Debug};

pub trait Summary {
    fn summarize(&self) -> String;

    fn summarize_author(&self) -> String;
} 

pub trait SummaryWithDefault {
    fn summarizeWithDefault(&self) -> String {
        String::from("(Read more...)")
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
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

impl SummaryWithDefault for NewsArticle {}

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

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

impl SummaryWithDefault for Tweet {
    fn summarizeWithDefault(&self) -> String {
       return String::from("Custom Tweet Summary")
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// this is the same as the above function

pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// multiple trait bounds
pub fn notify_1(item1: &impl Summary, item2: &impl Summary) {}
pub fn notify_2<T: Summary>(item1: &T, item2: &T) {}

// multiple trait bounds with +
pub fn notify_3(item: &(impl Summary + Display)) {}
pub fn notify_4<T: Summary + Display>(item: &T) {}

// where clause for clearer trait bounds
fn some_function_1<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) {}
fn some_function_2<T, U>(t: &T, u: &U)
    where T: Display + Clone,
          U: Clone + Debug
{}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("John Doe"),
        content: String::from(
            "Mojo tojo bojo jojo",
        ),
        reply: false,
        retweet: false,
    }
}
// ERROR: you cant return different types

// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from(
//                 "Penguins win the Stanley Cup Championship!",
//             ),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from(
//                 "of course, as you probably already know, people",
//             ),
//             reply: false,
//             retweet: false,
//         }
//     }
// }

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
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}



fn main() {
    let news_article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best
            hockey team in the NHL."),
    };
    let tweet = Tweet {
        username: String::from("RocketRaccoon"),
        content: String::from("This is my first tweet!"),
        reply: false,
        retweet: false,
    };
    println!("News Article :\n{}", news_article.summarize());
    println!("Tweet :\n{}", tweet.summarize());

    println!("---------------------");

    println!("News Article with default summary : {}", news_article.summarizeWithDefault());
    println!("Tweet with default summary (overrided) : {}", tweet.summarizeWithDefault());

    println!("---------------------");

    println!("News Article author : {}", news_article.summarize_author());
    println!("Tweet author : {}", tweet.summarize_author());

    println!("---------------------");

    notify(&news_article);
    notify(&tweet);

    println!("---------------------");

    returns_summarizable();

    println!("---------------------");

    let pair = Pair::new(21, 2);
    pair.cmp_display();
}
