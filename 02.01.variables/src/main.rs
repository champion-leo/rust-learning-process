use colored::Colorize;
const THREE_HOURS_IN_MILLISECONDS: u32 = 3 * 60 * 60 * 1000;

fn main() {
    print_title("Mutability:");
    mutable();
    print_title("Constant:");
    println!("{THREE_HOURS_IN_MILLISECONDS}");
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