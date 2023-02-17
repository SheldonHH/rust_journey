use std::thread;
use std::sync::mpsc;
// mpsc 多生产者，单消费者
// spmc：单生产者，多消费者
// 创建
// let (tx,rx) = mpsc::channel();
// let (tx,rx) = spmc::channel();
fn main() {
    // mpsc::channel例子
    // let (发送端，接收端)
    let (tx,rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Receive message: {}", received);

}


// 知识点：
// 1. 发送者的send方法，返回值为Result<T,E>, 若接收端已经被丢弃，将没有发送值的目标，此时发送会返回错误
// 2. 接受者的Receive，返回值为Result<T,E>,通道关闭，则返回错误
// 3. 接收端的recv会阻塞，但try_recv()不会阻塞，会立即返回