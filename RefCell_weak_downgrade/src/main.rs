/*
弱引用Weak
1. Rc::downgrade传递Rc实力引用，调用Rc::downgrade会得到类似Weak<T>的智能指针，同时将weak_count+1（strong_count不变）
2. weak_count无需计数为0就能使得Rc实例被清理，只要strong count为0即可
3. 可通过Rc::upgrade 方法返回Option<Rc<T>>  对象

*/
#[derive(Debug)]
enum List{
    Cons(i32, RefCell<Weak<List>>),
    Nil,
}

use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;
use crate::List::{Cons,Nil};
impl List{
    fn tail(&self) -> Option<&RefCell<Weak<List>>>{
        match self{
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}


fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Weak::new())));
    println!("1, strong count = {}, weak count = {}", Rc::strong_count(&a), Rc::weak_count(&a)); // 1, 0
    println!("1, a tail = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Weak::new())));
    if let Some(link) = b.tail(){
        *link.borrow_mut() = Rc::downgrade(&a); //🌟 勿忘()
    }

    println!("2, a strong count = {}, weak count = {}", Rc::strong_count(&a), Rc::weak_count(&a)); // a: 1,1
    println!("2, b strong count = {}, weak count = {}", Rc::strong_count(&b), Rc::weak_count(&b)); // b: 1, 0
    println!("2, a tail = {:?}", b.tail());

    if let Some(link) = a.tail(){
        *link.borrow_mut() = Rc::downgrade(&b); //🌟 勿忘()
    }

    println!("3, a strong count = {}, weak count = {}", Rc::strong_count(&a), Rc::weak_count(&a));
    println!("3, b strong count = {}, weak count = {}", Rc::strong_count(&b), Rc::weak_count(&b));
    println!("3, a tail = {:?}", b.tail());

    println!("Hello, world!");
}
