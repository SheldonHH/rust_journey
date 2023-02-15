// 1. Drop trait类似析构函数，当值离开作用域时执行此函数的代码
struct Dog{
    name: String,
    count: i32,
}

impl Drop for Dog{
    fn drop(&mut self){ // 注意：是【可变引用】，rust要释放，所以要修改🌟
        println!("Dog {} leave",self.name);
        self.count -= 1;
    }
}

fn main() {
    let a = Dog{name: String::from("Andy"), count: 100};
    {
        let b = Dog{name: String::from("Bob"), count: 240};
        println!("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb")
    }
    println!("1111111111111111111111111111111111111111111111111111111");
    println!("Hello, world!");
}
