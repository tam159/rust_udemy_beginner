use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::thread::sleep;

#[allow(unused_assignments)]
#[allow(unused_variables)]

const NUM_THREADS: usize = 3;

pub fn example_threads() {
    let mut threads = vec![];

    for i in 0..5 {
        let th = thread::spawn(move || {
            sleep(std::time::Duration::from_millis(10));
            println!("Thread {}", i);
        });
        threads.push(th)
    }

    for t in threads {
        t.join().unwrap();
    }

    println!("Main thread")
}

pub fn example_channel() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        tx.send("Hello from the thread!").unwrap();
    });
    println!("Message received: {}", rx.recv().unwrap());
}

fn start_thread(i: usize, tx: mpsc::Sender<usize>) {
    thread::spawn(move || {
        println!("Setting timer {}", i);
        thread::sleep(std::time::Duration::from_millis(i as u64));
        println!("Sending {}", i);
        tx.send(i).unwrap();
    });
}

pub fn example_multiple_channels() {
    let (tx, rx) = mpsc::channel();
    for i in 0..NUM_THREADS {
        start_thread(i, tx.clone());
    }
    for j in rx.iter().take(NUM_THREADS) {
        println!("Received {}", j);
    }
}

pub fn example_mutex() {
    let c = Arc::new(Mutex::new(0));
    let mut threads = vec![];

    for i in 0..10 {
        let c = Arc::clone(&c);
        let th = thread::spawn(move || {
            let mut num = c.lock().unwrap();
            *num += 1;
            println!("Thread {} - value {}", i, num);
        });
        threads.push(th);
    }

    for th in threads {
        th.join().unwrap();
    }

    println!("Result: {}", *c.lock().unwrap());
}
