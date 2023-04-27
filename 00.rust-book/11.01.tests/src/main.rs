
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}", name)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

pub struct GuessWithDetailErrorMessage {
    value: i32,
}

impl GuessWithDetailErrorMessage {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    
   use super::{Rectangle, greeting, Guess, GuessWithDetailErrorMessage};


    #[test]
    fn exploration() {
        let result = 2 + 2;
        assert_eq!(result, 4);
        assert_ne!(5, 2 + 2);
    }

    #[test]
    fn another() {
        // panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let rec1 = Rectangle {
            width: 10,
            height: 10
        };
        let rec2 = Rectangle {
            width: 5,
            height: 5
        };
        assert!(rec1.can_hold(&rec2));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let rec1 = Rectangle {
            width: 5,
            height: 5
        };
        let rec2 = Rectangle {
            width: 10,
            height: 10
        };
        assert!(!rec1.can_hold(&rec2));
    }


    /**
     * Error with custom failing message
     */
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100_specific_error() {
        GuessWithDetailErrorMessage::new(200);
    }

    // Using Result<T, E> in tests
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    
}

fn main() {
    println!("Hello, world!");
}
