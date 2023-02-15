// enum List{
//     Cons(i32, Box<List>),
//     Nil,
// }

enum List{
    Cons(i32, Rc<List>),
    Nil,
}
use crate::List::{Cons, Nil};
use std::rc::Rc; //🌟
fn main( ) {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil))))); //注意和下面的区别
    // let a = Cons(5,Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a)); // 当a被move到b之后，无法被c reference
    let b = Cons(3, Rc::clone(&a)); //🌟 两种clone写法 Rc::clone(&a)
    let b = Cons(3, a.clone());     //🌟 两种clone写法 a.clone()
    // let c = Cons(4, Box::new(a));
    let c = Cons(4, Rc::clone(&a));
    println!("Hello, world!");
}


/*
  |
7 |     let a = Cons(5,Box::new(Cons(10, Box::new(Nil))));
  |         - ❌ move occurs because `a` has type `List`, which does not implement the `Copy` trait
8 |     let b = Cons(3, Box::new(a));
  |                              - value moved here
9 |     let c = Cons(4, Box::new(a));
  |                              ^ value used here after move
*/