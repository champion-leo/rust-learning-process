// Using the Newtype Pattern for Type Safety and Abstraction
// check previous chapter for more details
// and chapter 17.1

fn main() {
    // Creating Type Synonyms with Type Aliases
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    // Box<dyn Fn() + Send + 'static>

    // let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));

    // fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
    //     // --snip--
    // }

    // fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
    //     // --snip--
    // }

    // type Thunk = Box<dyn Fn() + Send + 'static>;

    // let f: Thunk = Box::new(|| println!("hi"));

    // fn takes_long_type(f: Thunk) {
    //     // --snip--
    // }

    // fn returns_long_type() -> Thunk {
    //     // --snip--
    // }

    use std::fmt;
    use std::io::Error;

    pub trait Write {
        fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
        fn flush(&mut self) -> Result<(), Error>;

        fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
        fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
    }

    type Result2<T> = std::result::Result<T, Error>;

    pub trait Write2 {
        fn write(&mut self, buf: &[u8]) -> Result2<usize>;
        fn flush(&mut self) -> Result2<()>;

        fn write_all(&mut self, buf: &[u8]) -> Result2<()>;
        fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result2<()>;
    }

    // The Never Type that Never Returns

    fn bar() -> ! {
        loop {
            println!("Hello, world!");
        }
    }

    // Dynamically Sized Types and the Sized Trait

    fn generic<T>(t: T) {
        // --snip--
    }

    fn generic2<T: Sized>(t: T) {
        // --snip--
    }

    // relax the restriction on Sized by using the following code
    fn generic3<T: ?Sized>(t: &T) {
        // --snip--
    }
}
