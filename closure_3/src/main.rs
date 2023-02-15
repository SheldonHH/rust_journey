fn main() {
    // let x = 4;
    // let equal_to_x = |z| z==x; //x为环境
    // let y = 4;
    // assert!(equal_to_x(y));

    let x = vec![1,2,3];
    // let equal_to_x = |z| z == x; //✅
    let equal_to_x = move |z| z == x;     // 🌟🌟🌟 move关键字把所有权移动到闭包中 🌟🌟🌟
    println!("x = {:?}",x); //❌  ^ value borrowed here after move，所有权已转移

    let y = vec![1,2,3];
    assert!(equal_to_x(y));
}
 

/*
三种方式获取环境参数
1. 所有权
2. mut borrow 
3. none-mut borrow

三个Fn trait
FnOnce消费从周围作用域捕获变量，closure的scope称为环境。
1. 必须消费变量，并将所有权移入closure；其名称的Once部分，代表Closure≠多次获取相同变量的所有权
2. FnMut获取mut borrow
3. 所有闭包可被调用at least once，因此所有闭包都实现FnOnce
没有移动被捕获变量的所有权得到闭包也实现了FnMut，而不需要对捕获的变量进行可变访问的闭包实现Fn
*/