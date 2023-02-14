extern crate crypto; // 导入
// learn crate4 https://www.bilibili.com/video/BV1xJ411B79h?t=181.4&p=22
use crypto::digest::Digest;
use crypto::sha3::Sha3;
fn main() {
    let mut hasher = Sha3::sha3_256();
    hasher.input_str("hello world");
    let result = hasher.result_str();

    // 
    println!("hash = {}", result);
    println!("Hello, world!");
}

// 会自动下载