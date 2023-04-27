fn main() {
    basic_reference_example();
    mutable_reference();
}

fn basic_reference_example() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

/*
    fn update_reference_error() {
        let s = String::from("hello");

        change(&s);
    }

    fn change(some_string: &String) {
        some_string.push_str(", world");
    }
*/

fn mutable_reference() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("{}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

/*
    At any given time, you can have either one mutable reference or any number of immutable references.
  
        let mut s = String::from("hello");

        let r1 = &mut s;
        let r2 = &mut s; // ERROR

        println!("{}, {}", r1, r2);

    -----------------------------

        let mut s = String::from("hello");

        let r1 = &s; // no problem
        let r2 = &s; // no problem
        let r3 = &mut s; // BIG PROBLEM

        println!("{}, {}, and {}", r1, r2, r3);
 */

/*
    Rust forbids us from creating a dangling reference, which is a reference to a value that is no longer valid.
    
    fn main() {
        let reference_to_nothing = dangle();
    }

    fn dangle() -> &String {
        let s = String::from("hello");

        &s
    }
 */