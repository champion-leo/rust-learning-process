mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

use std::collections::HashMap;

// bring two type into scope with same name
// use std::fmt;
// use std::io;

// fn function1() -> fmt::Result {
//     // --snip--
// }

// fn function2() -> io::Result<()> {
//     // --snip--
// }

// --------------------------------

// as keyword to create alias
// use std::fmt::Result;
// use std::io::Result as IoResult;

// fn function1() -> Result {
//     // --snip--
// }

// fn function2() -> IoResult<()> {
//     // --snip--
// }

// --------------------------------

// Re-exporting Names with pub use
// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// pub use crate::front_of_house::hosting;

// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
// }

// Multiple use in one line

// use std::cmp::Ordering;
// use std::io;
// equivalent to
// use std::{cmp::Ordering, io};

// use std::io;
// use std::io::Write;
// equivalent to
// use std::io::{self, Write};


// glob operator to bring all public definitions into scope
// ! not recommended
// mostly used for tests files where you want to bring everything into scope
// use std::collections::*;


use rand::Rng;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
    eat_at_restaurant();
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {}", secret_number);
}
