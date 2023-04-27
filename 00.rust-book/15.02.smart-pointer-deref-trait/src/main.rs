fn main() {
    let x = 5;
    let y = &x;
    let z = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
    // assert_eq!(5, z); ERROR: can't compare `{integer}` with `&{integer}`
    assert_eq!(y, z);
    println!("x: {}, y: {}, z: {}", x, y, z);
    println!("x: {}, *y: {}, *z: {}", x, *y, *z);

    let box_x = Box::new(x);
    assert_eq!(5, *box_x);

    // Defining our own smart pointer
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    // assert_eq!(5, *y); // ERROR: type `MyBox<{integer}>` cannot be dereferenced
    use std::ops::Deref;
    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &T {
            &self.0
        }
    }

    assert_eq!(5, *y); // no error

    // Implicit Deref Coercion

    fn hello(name: &str) {
        println!("Hello, {}!", name);
    }
    let m = MyBox::new(String::from("Rust"));

    hello(&m); // no error

    // Same without Deref coercion
    hello(&(*m)[..]);

    hello(&MyBox::new(String::from("Rust")));
    // MyBox -> Deref -> &String -> Deref -> &str
}
