// 1. 进程：是资源分配的最小单位，线程数CPU调度的最小单位
// 2. 多线程
// 2.1 竞争状态：多个线程不一致顺序访问数据或资源
// 2.2 死锁🔒：相互等待
// 3, golang提供绿色线程：底层实现M:N模型，M绿色线程对应N个OS线程
// Rust仅提供1:1线程模型

use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(||{ //闭包
        for i in 1..10{ 
            println!("num {}",i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5{

    }
    println!("Hello, world!");
}
