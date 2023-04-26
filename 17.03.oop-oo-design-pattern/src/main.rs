// use oop_oo_design_pattern::Post;
// fn main() {
//     let mut post = Post::new();
//     post.add_text("I ate a salad for lunch today");
//     assert_eq!("", post.content());

//     post.request_review();
//     assert_eq!("", post.content());

//     post.approve();
//     assert_eq!("I ate a salad for lunch today", post.content());
// }

// Second example with compiler error to use rust type system
// to have an error at compile time instead of runtime

pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    // post.content(); ERROR: method not found in `DraftPost`

    let post = post.request_review();

    // post.content(); ERROR: method not found in `PendingReviewPost`

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}
// from the book: https://doc.rust-lang.org/stable/book/ch17-03-oo-design-patterns.html
// The changes we needed to make to main to reassign post mean that this implementation
// doesn’t quite follow the object-oriented state pattern anymore: the transformations
// between the states are no longer encapsulated entirely within the Post implementation.
// However, our gain is that invalid states are now impossible because of the type system
// and the type checking that happens at compile time! This ensures that certain bugs,
// such as display of the content of an unpublished post, will be discovered before
// they make it to production.

// Try the tasks suggested at the start of this section on the blog crate as it is after
// Listing 17-21 to see what you think about the design of this version of the code.
// Note that some of the tasks might be completed already in this design.

// We’ve seen that even though Rust is capable of implementing object-oriented design patterns,
// other patterns, such as encoding state into the type system, are also available in Rust.
// These patterns have different trade-offs. Although you might be very familiar with
// object-oriented patterns, rethinking the problem to take advantage of Rust’s features
// can provide benefits, such as preventing some bugs at compile time. Object-oriented patterns
// won’t always be the best solution in Rust due to certain features, like ownership,
// that object-oriented languages don’t have.
