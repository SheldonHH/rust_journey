use std::ops::Deref; // 🌟从标准库中实现解引用操作
struct MyBox<T>(T);

impl<T> MyBox<T>{
    fn new(x: T) -> MyBox<T>{
        MyBox(x)
    }
}

//实现trait
// 🌟无需获取ownership，只需使用，所以返回引用 &self.0，给MyBox实现Deref trait🌟
impl<T> Deref for MyBox<T>{
    type Target = T;
    fn deref(&self) -> &T{
        &self.0
    }
}
fn main() {
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5,x);
    assert_eq!(5,*y); //✅❌ type `MyBox<{integer}>` cannot be dereferenced
    println!("Hello, world!");

    println!("+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++");
    // 解引用强制多态
    // 1. 将MyBox变为&String
    // 2. 再将String的解引用，变为字符串slice
    // 3. 最后变成 &str
    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    // 解引用多态和可变性交互
    // 1. 当T: Deref<Target=U>时，从&T到&U
    // 2. 当T: DerefMut<Target=U>时，从&mut T到 &mut U  (双mut)
    // 3. 当T: Deref<Target=U>时，从&mut T到&
}


fn hello(name: &str){
    println!("Hello, {}",name);
}