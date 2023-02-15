enum List{
    Cons(i32, Box<List>),
    Nil,
}
use crate::List::{Cons, Nil};
fn main() {
    let a = Cons(5,Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(a));
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