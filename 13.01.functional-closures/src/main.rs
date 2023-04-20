use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveway1 = store.giveaway(user_pref1);
    println!("user {:?} giveway {:?}", user_pref1, giveway1);

    let user_pref2 = None;
    let giveway2 = store.giveaway(user_pref2);
    println!("user {:?} giveway {:?}", user_pref2, giveway2);

    // Capturing references or moving ownership
    // Only borrows
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    // Borrows and mutates
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    // ERROR
    // list is currently borrowed
    // println!("After defining and before calling closure: {:?}", list);

    borrows_mutably();
    println!("After calling closure: {:?}", list);

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    // impl<T> Option<T> {
    //     pub fn unwrap_or_else<F>(self, f: F) -> T
    //     where
    //         F: FnOnce() -> T
    //     {
    //         match self {
    //             Some(x) => x,
    //             None => f(),
    //         }
    //     }
    // }

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 5,
            height: 5,
        },
        Rectangle {
            width: 1,
            height: 10,
        },
    ];

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);

    // let mut list = [
    //     Rectangle {
    //         width: 10,
    //         height: 1,
    //     },
    //     Rectangle {
    //         width: 3,
    //         height: 5,
    //     },
    //     Rectangle {
    //         width: 7,
    //         height: 12,
    //     },
    // ];

    // let mut sort_operations = vec![];
    // let value = String::from("by key called");

    // list.sort_by_key(|r| {
    //     // ERROR
    //     // cannot move out of `value`, a captured variable in an `FnMut` closure
    //     // move occurs because `value` has type `String`, which does not implement the `Copy`
    //     sort_operations.push(value);
    //     r.width
    // });
    // println!("{:#?}", list);

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);
}
