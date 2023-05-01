// Specifying Placeholder Types in Trait Definitions with Associated Types
// pub trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }

pub struct Counter {}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        return Some(1);
    }
}

// Default Generic Type Parameters and Operator Overloading

// trait Add<Rhs=Self> {
//     type Output;

//     fn add(self, rhs: Rhs) -> Self::Output;
// }

use std::ops::{Add, Deref};

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug)]
struct Millimeters(u32);
struct Meters(u32);

impl std::cmp::PartialEq for Millimeters {
    fn eq(&self, other: &Millimeters) -> bool {
        self.0 == other.0
    }
}

impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

// Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// Using Supertraits to Require One Trait’s Functionality Within Another Trait

use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

#[derive(Debug)]
struct Point2 {
    x: i32,
    y: i32,
}

impl fmt::Display for Point2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point2 {}

// Using the Newtype Pattern to Implement External Traits on External Types

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

impl OutlinePrint for Wrapper {}

impl Deref for Wrapper {
    type Target = Vec<String>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    assert_eq!(Millimeters(1000) + Meters(1), Millimeters(2000));

    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
    // same as
    Human::fly(&person);

    println!("A baby dog is called a {}", Dog::baby_name());
    // fully qualified syntax to ensure we call the method from the Animal trait
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    let p = Point2 { x: 1, y: 2 };
    p.outline_print();

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
    w.outline_print();
}
