fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
    // The exhaustive match is not necessary with if let syntax
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    } else {
        println!("The maximum is not configured");
    }
}
