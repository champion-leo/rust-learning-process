use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    // handle.join().unwrap();
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    // this line will wait for the spawned thread to finish
    // otherwise the main process will be stop and all child process
    // will be killed
    // handle.join().unwrap();

    let v = vec![1, 2, 3];
    // use move to pass the ownership of v to the closure
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    // println!("v: {:?}", v); ERROR value borrowed here after move
    handle.join().unwrap();
}
