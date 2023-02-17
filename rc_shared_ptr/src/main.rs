enum List{
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;
// 通过Rc<T>允许程序多个部分之间只读的共享数据，因为相同位置的多个可变引用可能会造成数据竞争的不一致
// strong_count: Gets the number of strong (Rc) pointers to this allocation.
fn main() {
    let a = Rc::new(Cons(5,Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));

    let b = Cons(3, Rc::clone(&a));
    // println!("count after creating b = {}", Rc::strong_count(&b)); 
    // ❌  ^^ expected struct `Rc`, found enum `List`
    println!("count after creating b, a = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c, a = {}", Rc::strong_count(&a));
    }// 🌟 离开作用域 Rc-1

    println!("count at end a = {}", Rc::strong_count(&a));
    println!("Hello, world!");
}
