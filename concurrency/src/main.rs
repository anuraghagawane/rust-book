use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    // handle.join().unwrap();
    for i in 1..5 {
        println!("hi number {i} from the main thread");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    let v = vec![1, 2, 3];

    // we have to move the value cuz the value might be deleted or dropped
    // in main thread which will cause problem in spawn thread as it is 
    // holding reference to that value
    let handle = thread::spawn(move || {
        println!("here's a vector: {v:?}");
    });

    handle.join().unwrap();


    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // println!("val is {val}"); // cannot do this after value is transmitterd
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
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
        println!("Got: {received}");
    }
}

