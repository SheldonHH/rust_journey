use std::thread;
use std::sync::mpsc;
fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        println!("val = {}",val);
    })

    let re =rx.recv().unwrap();
    println!("got:{}",re)
    println!("Hello, world!");
}
