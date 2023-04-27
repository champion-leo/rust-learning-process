fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("IpAddrKind::V4: {:?}", four);
    println!("IpAddrKind::V6: {:?}", six);
    println!("------------------");

    let home = IpAddrStruct {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    
    let loopback = IpAddrStruct {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("home: {:?}", home);
    println!("loopback: {:?}", loopback);
    println!("------------------");

    let home = IpAddrEnum::V4(String::from("127.0.0.1"));
    let loopback = IpAddrEnum::V6(String::from("::1"));
    println!("home enum: {:?}", home);
    println!("loopback enum: {:?}", loopback);
    println!("------------------");

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("home enum different types: {:?}", home);
    println!("loopback enum different types: {:?}", loopback);
    println!("------------------");

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
    println!("some_number: {:?}", some_number);
    println!("some_char: {:?}", some_char);
    println!("absent_number: {:?}", absent_number);
    println!("------------------");


    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let y2 = match y {
        Some(i) => i,
        None => 0,
    };
    // let sum = x + y; // error: no implementation for `i8 + std::option::Option<i8>`
    println!("sum: {:?}", x + y2);
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddrStruct {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddrEnum {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Message: {:?}", self)
    }
}

