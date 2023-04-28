static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;
fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
fn main() {
    // Raw pointers
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    println!("r1 is: {:?}", r1);
    println!("r2 is: {:?}", r2);

    // You need to use unsafe to dereference raw pointers
    unsafe {
        println!("dereferenced r1 is: {}", *r1);
        println!("dereferenced r2 is: {}", *r2);
        *r2 = 6;
        println!("dereferenced r2 is: {}", *r2);
    }

    // Not valid raw pointer

    let address = 0x012345usize;
    let r = address as *const i32;

    println!("r is: {:?}", r);

    // unsafe {
    //     // Segmentation fault crash
    //     println!("dereferenced r is: {}", *r);
    // }

    // Calling unsafe functions
    unsafe fn dangerous() {}
    unsafe {
        dangerous();
    }

    // Creating safe abstraction over unsafe code
    use std::slice;
    fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();
        let ptr = values.as_mut_ptr();

        assert!(mid <= len);

        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = split_at_mut(r, 3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    let address = 0x01usize;
    let r = address as *mut i32;
    // Will cause a crash because the pointer have a high chance to be not valid
    // let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
    // println!("values is: {:?}", values);

    // Using extern functions to call external code
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("abs(-3) is: {}", abs(-3));
    }

    // Accessing or modifying mutable static variables
    println!("name is: {}", HELLO_WORLD);

    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    // Unsafe traits

    unsafe trait Foo {
        // methods go here
    }

    unsafe impl Foo for i32 {
        // method implementations go here
    }
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
