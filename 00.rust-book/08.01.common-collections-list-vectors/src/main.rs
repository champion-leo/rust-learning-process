fn main() {
    // Creating a new vector
    let v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];
    println!("v1: {:?}", v1);
    println!("v2: {:?}", v2);

    // Updating a vector
    let mut v3 = Vec::new();
    v3.push(1);
    v3.push(2);
    v3.push(3);
    println!("v3: {:?}", v3);

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // Iterating over the values in a vector

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
    println!("v: {:?}", v);

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("v: {:?}", v);

    // Storing an emu

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("row: {:?}", row);

    // Dropping a vector drops its elements
    {
        let v = vec![1, 2, 3, 4];
        println!("v: {:?}", v);
        // do stuff with v
    } // <- v goes out of scope and is freed here

    // Additional testing
    let mut v = vec![3, 6, 2, 9, 5];
    v.insert(1, 2);
    v.remove(1);
    v.sort();
    println!("v: {:?}", v);

}
