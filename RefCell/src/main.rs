 // 使用不可变引用的时候，改变数据
 // 2. 通过RefCell<T>在【运行时】检查借用规则 🌟
 // 3. 代表数据的唯一所有权
 // 4. 仅适用单线程场景，并非线程安全

 /*
 Rc<T>允许相同数据多个所有者：Box<T>和RefCell<T>有单一所有者
 Box<T> 编译时执行不可变+可变借用检查:  类似constexp
 Rc<T>仅运行时 🌟执行不可变+可变借用检查
 - 在RefCell<T>运行时执行可变借用检查，在自身不可变的情况下修改内部值
  */
#[derive(Debug)]
enum List{
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}
use crate::List::{Cons,Nil};
use std::rc::Rc;
use std::cell::RefCell;
fn main() {
    let value = Rc::new(RefCell::new(5)); 
    // let a = Rc::new(Cons(Rc::clone(&value)), Rc::new(Nil)); // ❌
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));  // ✅ Cons() 内部两个元素，一个时Rc::clone(&value)，另一个是Rc::new(Nil)
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(7)), Rc::clone(&a));


    println!("a after: {:?}",a);
    println!("b after: {:?}",b);
    println!("c after: {:?}",c);
    // 🌟RefCell<T>运行时执行可变借用检查，在自身不可变的情况下修改内部值🌟
    println!("++++++++++++++++++++++++++++++++++++++++++"); 
    *value.borrow_mut() += 10;
    println!("a after: {:?}",a);
    println!("b after: {:?}",b);
    println!("c after: {:?}",c);
    println!("Hello, world!");
}
