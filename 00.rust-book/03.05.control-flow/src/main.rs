fn main() {
    println!("More than five:");
    more_than_five(4);
    println!("Number divisibility:");
    number_divisibility(3);
    println!("Count to ten:");
    count_to_ten();
    println!("Loop label:");
    loop_label();
    println!("Conditional loop:");
    conditional_loop();
    println!("For in:");
    for_in();
    println!("For range rev:");
    for_range_rev();
}

fn more_than_five(number: i32) {
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn number_divisibility(number: i32) {
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn count_to_ten() {
    let mut counter = 0;

    let result = loop {
        println!("counter: {}", counter);
        counter += 1;

        if counter == 10 {
            break counter;
        }
    };

    println!("The final count is: {}", result);
}

fn loop_label() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 5 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn conditional_loop() {
    let mut number = 10;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_in() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

fn for_range_rev() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}