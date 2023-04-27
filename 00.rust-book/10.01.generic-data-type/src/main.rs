fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
// Error
// binary operation `>` cannot be applied to type `&T`
// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct TwoTypePoint<T, U> {
    x: T,
    y: U,
}

impl<X1, Y1> TwoTypePoint<X1, Y1> {
    fn mixup<X2, Y2>(self, other: TwoTypePoint<X2, Y2>) -> TwoTypePoint<X1, Y2> {
        TwoTypePoint {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    //let number_list = vec![34, 50, 25, 100, 65];
    // let result = largest(&number_list);
    // println!("The largest number is {}", result);

    // let char_list = vec!['y', 'm', 'a', 'q'];

    // let result = largest(&char_list);
    // println!("The largest char is {}", result);

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("integer.x = {}, integer.y = {}", integer.x, integer.y);
    println!("float.x = {}, float.y = {}", float.x, float.y);
    // Error expected integer, found floating-point number
    // let wont_work = Point { x: 5, y: 4.0 };
    let mix = TwoTypePoint { x: 5, y: 4.0 };
    println!("mix.x = {}, mix.y = {}", mix.x, mix.y);
    // generic method
    println!("integer.x = {}", integer.x());

    let p = Point { x: 5.5, y: 10.4 };
    println!("p.distance_from_origin() = {}", p.distance_from_origin());
    let p = Point { x: 5, y: 10 };
    // Error no method named `distance_from_origin` found for type `Point<i32>` in the current scope
    // println!("p.distance_from_origin() = {}", p.distance_from_origin());

    let p1 = TwoTypePoint { x: 5, y: 10.4 };
    let p2 = TwoTypePoint { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // Monomorphization at compile time allows the compiler to perform static dispatch
    // let integer = Some(5);
    // let float = Some(5.0);

    // -------------------

    // compiler will generate code for each type
    // enum Option_i32 {
    //     Some(i32),
    //     None,
    // }
    
    // enum Option_f64 {
    //     Some(f64),
    //     None,
    // }
    
    // fn main() {
    //     let integer = Option_i32::Some(5);
    //     let float = Option_f64::Some(5.0);
    // }
    
}
