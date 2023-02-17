use std::thread;
fn main() { 
//✅，正确运行需要move 
    let v = vec![1,2,3];

    // 这里无法保证v始终有效
    let handle = thread::spawn(move|| { //move将
        println!("v: {:?}",v);
    });

    // drop(v);

    // 尝试打印v
    // println!("v: {:?}",v); //❌ use after move
    handle.join().unwrap();
    println!("Hello, world!");
}
