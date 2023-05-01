// Macro

// Custom #[derive] macros that specify code added with the derive attribute used on structs and enums
// Attribute-like macros that define custom attributes usable on any item
// Function-like macros that look like function calls but operate on the tokens specified as their argument

// Declarative Macros with macro_rules! for General Metaprogramming

#[macro_export]
macro_rules! simplified_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// Procedural Macros for Generating Code from Attributes
// How to Write a Custom derive Macro
use hello_macro::HelloMacro;

// #[proc_macro]
// pub fn sql(input: TokenStream) -> TokenStream {}

// const SQL = sql!(SELECT * FROM posts WHERE id=1);

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

fn main() {
    let vec = simplified_vec![1, 2, 3];
    Pancakes::hello_macro();
}
