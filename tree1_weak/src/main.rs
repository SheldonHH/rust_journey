use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node{
    value: i32,
    parent: RefCell<Weak<Node>>,
    kids: RefCell<Vec<Rc<Node>>>
}

fn main() {
    let leaf = Rc::new( Node{
        value:3,
        parent: RefCell::new(Weak::new()),
        kids: RefCell::new(vec![]),
    })

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("Hello, world!");
}
