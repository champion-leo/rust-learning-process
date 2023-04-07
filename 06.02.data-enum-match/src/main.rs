fn main() {
    println!("Hello, world!");
    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let dime = Coin::Dime;
    let quarter = Coin::Quarter;
    println!("Penny: {}", value_in_cents(penny));
    println!("Nickel: {}", value_in_cents(nickel));
    println!("Dime: {}", value_in_cents(dime));
    println!("Quarter: {}", value_in_cents(quarter));
    println!("---------------------");
    let alabama = UsState::Alabama;
    let quarter = CoinWithUsState::Quarter(alabama);
    println!("Quarter: {}", value_in_cents_with_us_dollar(quarter));
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!(
        "five: {:?}, six: {:?}, plus_one(none): {:?}",
        five, six, none
    );
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    fn add_fancy_hat() {
        println!("add_fancy_hat");
    }
    fn remove_fancy_hat() {
        println!("remove_fancy_hat")
    }
    fn reroll() {
        println!("reroll")
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum CoinWithUsState {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents_with_us_dollar(coin: CoinWithUsState) -> u8 {
    match coin {
        CoinWithUsState::Penny => 1,
        CoinWithUsState::Nickel => 5,
        CoinWithUsState::Dime => 10,
        CoinWithUsState::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
