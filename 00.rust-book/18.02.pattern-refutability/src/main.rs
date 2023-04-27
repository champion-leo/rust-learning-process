fn main() {
    let some_option_value: Option<i32> = Some(5);
    // let Some(x) = some_option_value; // refutable pattern in local binding

    if let Some(x) = some_option_value {
        println!("{}", x);
    }
    // Here we have a warning because the inside of the if is always executed
    // and the if let is not necessary
    if let x = 5 {
        println!("{}", x);
    };
}
