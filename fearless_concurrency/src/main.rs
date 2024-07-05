use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;

fn main() {

    /* 
    Execution of two threads. These threads will _usually_ take turns, but this depends on how
    your particular OS handles threads. In this example, the spawned thread will usually only 
    reach count 5. 
     */
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    /*
    Execution of two threads using a join handle. This will ensure that execution will not conclude
    until both threads are done. 
    */

    println!("\n\n ====== THREADS USING JOIN HANDLES ======\n");
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });


    // Uncomment this to execute the entire spawn thread before the main thread.
    // handle.join().wrap();

    for i in 1..5 {
        println!("Hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    // Comment the below line if you have a join handle above. 
    handle.join().unwrap();

    let v = vec![1,2,3];

    // The move keyword is required for th closure to take ownership of vector v.
    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    // Note that this will not allow you to call drop on the main thread, as 
    // vector v now belongs to the spawned thread. 
    handle.join().unwrap();

    /*
    MULTI CHANNEL PASSING
     */

    // a Multiple Producer, Single Consumer channel. 
    // Basically, one end can recieve from many senders.

    let (tx, rx) = mpsc::channel();

    // Let's send this rubber ducky down the river.

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // The below line will cause the program to not compile,
        // as it's trying to call a value that's already transfered ownership.
        // println!("val is {val}");
    });

    // recv blocks the main thread's execution until a value is sent through the channel.
    let received = rx.recv().unwrap();
    println!("Got: {received}");

    /*
    Sending multiple values with receiver waiting. The code copying employed bugs me, but
    it's what the tutorial asks, so whatever. 

    Note that during execution, the strings from both producers are muddled together.
     */

    println!("\n==== TESTING MULTIPLE PRODUCER SENDS ====");

    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("there"),
            String::from("the"),
            String::from("spawned"),
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

    /*
    MUTEX (MUTual EXclusion)
    
    Demonstration of using Mutex to lock data to one given channel until released. */
    for received in rx {
        println!("Got: {received}");
    }

    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {m:?}");

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 1..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
