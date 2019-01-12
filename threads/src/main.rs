use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // can’t guarantee that the spawned thread will get to run at all or finish running
    // The return type of thread::spawn is JoinHandle. Calling join on the handle blocks the thread currently running until the thread represented by the handle terminates.
    handle.join().unwrap();

    let v = vec![1, 2, 3];

    // use the move keyword before the parameter list of a closure to force the closure to take ownership of the values it uses in the environment
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
