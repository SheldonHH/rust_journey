use std::thread;
use std::sync::mpsc;
use std::time::Duration;
// 双发送，单接受
fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);
    let tx2 = mpsc::Sender::clone(&tx);

    thread::spawn(move ||{
        let vals = vec![
            String::from("hi"),
            String::from("fsdf"),
            
            ];
            // for循环发送
            //暂用tx1发送
            for val in vals{
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }

    });

    thread::spawn(move ||{
        let vals = vec![
            String::from("A"),
            String::from("B"),
            String::from("C"),
            String::from("D"),
        ];
        // for循环发送
        //暂用tx2发送
        for val in vals{
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move ||{
        let vals = vec![
            String::from("1"),
            String::from("2"),
            String::from("3"),
            String::from("4"),
        ];
        // for循环发送
        //暂用tx2发送
        for val in vals{
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for rec in rx{
        println!("Got:{}", rec);
    }

    println!("Hello, world!");
}
