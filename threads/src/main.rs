use std::thread;
use std::time::Duration;
use std::sync::mpsc;


fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    let v = vec![1, 2, 3];

    let handle = thread::spawn( move || {   //move all variables into the closure env
        println!("Here's a vector: {:?}", v);
    });

    // drop(v); value v can't be dropped because it was moved inside the closure

    handle.join().unwrap();

    message_passing();

}

fn message_passing(){
    let (tx, rx) = mpsc::channel();  //mpsc - multiple producer single consumer

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
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