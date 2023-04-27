fn main() {
    // CREATE A NEW HASHMAP
    use std::collections::HashMap;
    let mut map = HashMap::new();
    map.insert(String::from("blue"), 10);
    map.insert(String::from("red"), 25);
    // overwriting a value
    map.insert(String::from("blue"), 100);
    println!("{:?}", map);

    // ACCESSING VALUES IN A HASHMAP
    let team_name = String::from("blue");
    let score = map.get(&team_name).copied().unwrap_or(0);
    println!("{}: {}", team_name, score);

    // ITERATING OVER HASHMAPS
    // the order of the key-value pairs is arbitrary
    println!("Iterating over hashmaps");
    for (key, value) in &map {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    // println!("{}, {}", field_name, field_value); // ERROR: value borrowed here after move

    // ADDING A NEW VALUE ONLY IF THE KEY HAS NO VALUE
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // UPDATING A VALUE BASED ON THE OLD VALUE
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

}
