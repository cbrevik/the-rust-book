use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Channels:
    // “Do not communicate by sharing memory; instead, share memory by communicating.”
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // The send-method takes ownership, so we cannot
        // use val after it is sent:
        // println!("val is {}", val);
        // once the value has been sent to another thread, that thread could modify or drop it before we try to use the value again
        // This prevents errors in concurrent programming
    });

    // a channel has two useful methods: recv and try_recv
    // recv blocks the thread, try_recv does not
    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    multiple_messages();
}

fn multiple_messages() {
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
