use std::thread;
use std::sync::{mpsc, Mutex, Arc};
use std::rc::Rc;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(||{
        for i in 1..10{
            println!("Hi number from the spawned thread {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5{
        println!("Hi from the main thread {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();

    // move keyword
    let v = vec![1, 2, 3];

    let m_handle = thread::spawn(move || {
        println!("Here's a vector {:?}", v);
    });

    m_handle.join().unwrap();

    // channels
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let recieved = rx.recv().unwrap();

    println!("From other thread got string {}", recieved);

    // multiple producers
    let (tx_mp, rx_mp) = mpsc::channel();

    let tx1 = tx_mp.clone();
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
            tx_mp.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx_mp {
        println!("Got: {}", received);
    }

    // shared state concurrency
    let mute = Mutex::new(5);
    {
        let mut num = mute.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", mute);

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let ct = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = ct.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

}
