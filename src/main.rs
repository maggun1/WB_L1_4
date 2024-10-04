use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let worker_count = std::env::args()
        .nth(1)
        .expect("Enter the number of workers!")
        .parse::<usize>()
        .expect("The number of workers must be a number!");

    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));

    let mut handles = vec![];

    for i in 0..worker_count {
        let rx = Arc::clone(&rx);
        let handle = thread::spawn(move || {
            loop {
                let data = rx.lock().unwrap().recv();
                match data {
                    Ok(value) => {
                        println!("Worker {} received data: {};", i, value);
                    }
                    Err(_) => {
                        println!("Worker {} is terminating.", i);
                        break;
                    }
                }
            }
        });
        handles.push(handle);
    }

    let producer = thread::spawn(move || {
        let mut counter = 0;
        loop {
            counter += 1;
            if tx.send(counter).is_err() {
                println!("There was an error sending data!");
                break;
            }
            thread::sleep(Duration::from_secs(1));
        }
    });

    producer.join().unwrap();
    for handle in handles {
        handle.join().unwrap();
    }
}
