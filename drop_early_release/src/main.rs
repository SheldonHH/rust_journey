struct Dog{
    name: String,
    count: i32,
}
impl Drop for Dog{
    fn drop(&mut self){ // 注意：是【可变引用】，rust要释放，所以要修改🌟
        println!("{} leave",self.name);
        self.count -= 1;
    }
}
// 🌟 rust提供 std::mem::drop();
fn main() {
    let a = Dog{name: String::from("Andy"), count: 99};
    let b = Dog{name: String::from("Bob"), count: 1287};

    // b.drop(); //❌explicit destructor calls not allowed
    // drop(b);
    println!("+++++++++++++++++++++++");

    println!("Hello, world!");
}
/* 正常
+++++++++++++++++++++++
Hello, world!
Bob leave
Andy leave
 */

/* drop(b);之后的结果
Bob leave
+++++++++++++++++++++++
Hello, world!
Andy leave
*/