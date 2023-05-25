// threads3.rs

// #![feature(test)]
// extern crate test; 

use std::sync::{Arc, mpsc}; // (multiple producer, single consumer) channel. send & receiving between threads
use std::thread;
use std::time::{Duration, Instant};

// To use mpsc we use clone to create duplicate of the original sending end and .send in main()
struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: mpsc::Sender<u32>) -> () {
    let qc = Arc::new(q);
    // let qc1 = Arc::clone(&qc);
    // let qc2 = Arc::clone(&qc);
    let qc1 = qc.clone();
    let qc2 = qc.clone();
    let tx1 = tx.clone();
    let tx2 = tx.clone(); 
    thread::spawn(move || {
        for val in &qc1.first_half {
            println!("sending {:?}", val);
            tx1.clone().send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    

    thread::spawn(move || {
        for val in &qc2.second_half {
            println!("sending {:?}", val);
            tx2.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();
    let queue_length = queue.length;

    let start = Instant::now();
    send_tx(queue, tx);
    let duration = start.elapsed();

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }
    println!("Time elapsed in expensive_function() is: {:?}", duration);
    println!("total numbers received: {}", total_received);

    assert_eq!(total_received, queue_length)
    
}

/* hint
An alternate way to handle concurrency between threads is to use
a mpsc (multiple producer, single consumer) channel to communicate.
With both a sending end and a receiving end, it's possible to
send values in one thread and receive them in another.
Multiple producers are possible by using clone() to create a duplicate
of the original sending end.
 */

/* 
#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    
    #[bench]
    fn test1(b: &mut Bencher, c: &mut Bencher) {
        let (tx, rx) = mpsc::channel();
        let queue = Queue::new();
        let queue_length = queue.length;
        b.iter(||
        send_tx(queue, tx)
        );
        c.iter(||
            send_tx2(queue, tx)
        );
        println!("{} vs {}",a + b) 
    }
    
}
*/