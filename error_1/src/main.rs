// 1. 可恢复+不可恢复
// 2. 可恢复：未找到文件，Result<T,E>🌟
// 不可回复：panic
// RUST_BACKTRACE=1 cargo run // 打印完整堆栈

// 4. 
// enum Result<T,E>{
    // Ok(T),
    // Err(E),
// }

use std::fs::File;
fn main() {
    // let f = File::open("hello.txt");
    // let r = match f{
    //     Ok(file) => file,
    //     Err(error) => panic!("error: {:?}", error),
    // };

    // 简写1 
    // let f = File::open("hello.txt").unwrap();
    // 简写2
    let f = File::open("hello.txt").expect("Fail to opennn");
    panic!("crash");
    println!("Hello, world!");
}



