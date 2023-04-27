fn main() {
    let mut s = String::from("hello world");
    let word = first_word_without_slice(&s); // word will get the value 5
    println!("The first word end index is: {}", word);
    s.clear();
    println!("-------------------");

    // UTF-8 only
    let s = String::from("hello world");
    let hello = &s[..5]; // same as &s[0..5]
    let world = &s[6..]; // same as &s[6..11]
    println!("The first word is: {}", hello);
    println!("The second word is: {}", world);
    println!("-------------------");

    let s = String::from("hello world");
    let word = first_word(&s);
    // s.clear(); ERROR: cannot borrow `s` as mutable because it is also borrowed as immutable
    println!("The first word is: {}", word);
    println!("-------------------");

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("The slice is: {:?}", slice);
    assert_eq!(slice, &[2, 3]);
}

fn first_word_without_slice(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}