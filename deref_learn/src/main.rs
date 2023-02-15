// Deref trait允许重载引用运算符
// let a:A = A::new(); // 前提🌟：A类型必须实现Deref trait
// let b = &a;
// let c = *b; // 解引用


fn main() {             
    let x = 5;                  // 🌟x中已实现解引用
    let y = &x;
    assert_eq!(5,x);
    assert_eq!(5,*y);
                            // 🌟Box中已实现解引用
    let z = Box::new(x); //将x Copy到Box所在堆上的内存, z此时再指向该内存
    assert_eq!(5,*z); //判断解引用是否相等
    println!("Hello, world!");
}
