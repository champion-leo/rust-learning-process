use std::fs::File;
use std::io::{self, Read, ErrorKind};

// PROPAGATING ERRORS
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// SHORTER VERSION WITH ? OPERATOR

fn short_read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}


fn main() {
    let file_result = File::open("file.txt");
    match file_result {
        Ok(file) => println!("The file is: {:?}", file),
        Err(error) => 
            println!("Problem opening the file: {:?}", error),
    };
    
    File::open("file.txt");

    // match file_result {
    //     Ok(file) => println!("The file is: {:?}", file),
    //     Err(error) => match error.kind() {
    //         std::io::ErrorKind::NotFound => match File::create("file.txt") {
    //             Ok(fc) => println!("File created: {:?}", fc),
    //             Err(e) => println!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => println!("Problem opening the file: {:?}", other_error),
    //     }
    // }

    // CLOTHURE APPROACH

    File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    
    // UNWRAP AND EXPECT

    // expect() panics if the Result is an Err value, you can choose the panic message
    File::open("wow.text").expect("Failed to open wow.txt");

    // unwrap() panics if the Result is an Err value
    File::open("wow.text").unwrap();

    read_username_from_file();
    short_read_username_from_file();

}
// CHANGING MAIN TO RETURN A RESULT

// fn main() -> Result<(), Box<dyn Error>> {
//     let greeting_file = File::open("hello.txt")?;

//     Ok(())
// }