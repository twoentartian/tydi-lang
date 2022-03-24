extern crate spmc;
use std::thread;

fn main() {
    let (mut tx, rx) = spmc::channel();

    let mut handles = Vec::new();
    for n in 0..5 {
        let rx = rx.clone();
        handles.push(thread::spawn(move || {
            let msg = rx.recv().unwrap();
            println!("worker {} recvd: {}", n, msg);
        }));
    }

    for i in 0..50 {
        tx.send(i * 2).unwrap();
    }

    for handle in handles {
        handle.join().unwrap();
    }
}