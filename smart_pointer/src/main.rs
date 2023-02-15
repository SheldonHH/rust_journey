

/*
Deref
Drop trait当离开作用域
Box<T>值在堆上而非栈上 指向堆数据的指针，box性能无损伤
使用场景
1. 编译时未知大小，上下文使用时
2. 大量数据，希望在不Copy数据的情况下转移数据所有权
3. 只关注一个var的类型是否实现了特定trait而非具体类型
*/
// 定义枚举类型
// ❌ recursive type `List` has infinite size
// help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle
/* |
15 |     Cons(i32, Box<List>),
   |               ++++    +
*/
// enum List{
//     Cons(i32, List),
//     Nil,
// }

// C++中使用
// struct List{
//     int value;
//     struct List *next; // 指针大小确定✅，结构体大小固定
//     // struct List l; //❌存在递归
// };

enum List{
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    // e.g. 0
    // let b = Box::new(5);
    // // b在栈上，5在堆上，b指向5所在内存位置
    // println!("b = {}",b);
    // println!("Hello, world!");

    use List::Cons;
    use List::Nil;
    let list = Cons(1, 
                    Box::new(Cons(2, 
                            Box::new(Cons(3, 
                                Box::new(Nil))))));
    
}
