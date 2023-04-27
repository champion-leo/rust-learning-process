fn main() {
    // INITIALIZE A STRING

    let mut s = String::new();
    println!("Hello, world!");
    
    // to_string() is a method that converts a string literal to a String type
    // to_string() is available on all types that implement the Display trait
    let data = "initial contents";
    let s = data.to_string();
    println!("{}", s);

    let s = "initial contents".to_string();
    println!("{}", s);

    let s = String::from("initial value");
    println!("{}", s);

    let list_of_strings = vec![String::from("السلام عليكم"), String::from("안녕하세요")];
    println!("{:?}", list_of_strings);

    // APPEND A STRING

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 is {s1}");

    let mut s1 = String::from("lo");
    s1.push('l');
    println!("s1 is {s1}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    // s1 has been moved here and can no longer be used
    println!("s3 is {s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    let s = format!("{}-{}-{}", s1, s2, s3);
    // s1, s2, and s3 are still valid here because they were not moved into the format! macro

    // INDEXING INTO A STRING

    let hello = String::from("Hola");
    println!("{}", hello.len());
    let hello = String::from("Здра");
    println!("{}", hello.len());

    let hello = "Здравствуйте";

    let s = &hello[0..4];
    println!("{}", s);

    for c in "Зд".chars() {
        println!("{c}");
    }
    
    for b in "Зд".bytes() {
        println!("{b}");
    }
}
