struct Counter{
    count: u32,
}
impl Counter{
    fn new() -> Counter{
        Counter {
            count: 0
        }
    }
}

impl Iterator for Counter {
    //关键类型
    type Item = u32;
    //实现next(&mut self)
    fn next(&mut self) -> Option<Self::Item>{
        self.count += 1;
        if self.count < 6{
            Some(self.count)
        } else{
            None
        }
    }
}
fn main() {
    let mut counter = Counter::new();
    for i in 0..19 {
        if let Some(v) = counter.next(){
            println!("i={},v={}",i,v);
        }else{
            println!("i={}, at end",i);
            // break;
        }
    }
    println!("Hello, world!");
}
