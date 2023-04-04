use colored::Colorize;
const THREE_HOURS_IN_MILLISECONDS: u32 = 3 * 60 * 60 * 1000;

fn main() {
    print_title("Mutability:");
    mutable();
    print_title("Constant:");
    println!("{THREE_HOURS_IN_MILLISECONDS}");
    print_title("Shadowing:");
    shadowing();
}

fn print_title(title: &str) {
    println!("{}", title.bold().green());
}

fn mutable() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

fn shadowing() {
    let x = 5;
    println!("The value of x is initiate: {}", x);
    let x = x + 1;
    println!("The value of x is shadowed to x + 1: {}", x);
    {
        let x = x * 2;
        println!("The value of x is shadowed to x * 2 inside a scope {}", x);
    }
    println!("The value of x outside tye scope {}", x);
    let spaces = "   ";
    println!("");
    println!("The value of spaces is: '{}'", spaces);
    let spaces = spaces.len();
    println!("The value of spaces after using shadowind to change the type of the variable : {}", spaces);
}