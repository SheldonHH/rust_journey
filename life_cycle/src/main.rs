// 1.引用保持有效作用于
// 2. 避免悬垂引用
// 3. borrower checker检查声明周期是否有效
fn main() {
    //被引用的r生命周期小于引用者 /
    // let r;                                                          // ------------------ + ------'r
    // {                                                               //                    +
    //     let x = 5;                                                  // ---+----'x         |
    //     r = &x;                                                     //    |               |
    // } //x dropped here while still borrowed                         // ---+               |
    // println!("r = {}",r); //悬垂指针                                 //     -              |
    

    let r;                                     //r生               

    let x = 5;                          //x生               
    r = &x;     

    println!("r = {}",r);

    println!("Hello World!"); // r，x结束




}
