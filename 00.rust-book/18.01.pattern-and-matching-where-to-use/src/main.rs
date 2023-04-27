fn main() {
    let mut option = Some(5);
    match option {
        Some(x) => println!("x is {}", x),
        None => println!("x is None"),
    }
    match option {
        _ => println!("option is {:?}", option),
    }
    let value_or_set_default = match option {
        Some(x) => x,
        None => 0,
    };

    // If let
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();
    // if let is a less verbose way to write a match that only matches one case
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let x = 5;
    // let PATTERN = EXPRESSION;
    let (x, y, z) = (1, 2, 3);
    // let (x, y) = (1, 2, 3); ERROR
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }
    let point = (3, 5);
    print_coordinates(&point);
}
