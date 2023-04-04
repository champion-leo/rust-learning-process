const THREE_HOURS_IN_MILLISECONDS: u32 = 3 * 60 * 60 * 1000;

fn main() {
    println!("Mutable:");
    mutable();
    println!("Constant:");
    println!("{THREE_HOURS_IN_MILLISECONDS}");
}

fn mutable() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}