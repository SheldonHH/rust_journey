// 类似c/ c++的全局变量 extern
// 所有string literal都有static 生命周期
// let s: &'static str = "hello";


use std::fmt::Display;
// 🌟对泛型约束，要求实现Display的特征🌟
fn function<'a,T:Display>(x: &'a str, y: &'a str, ann: T) -> &'a str{
    println!("ann is {}",ann);
    if x.len() < y.len(){
        x
    }else{
        y
    }
}

fn main() {
    let s1 = String::from("i am s1");
    let s2 = String::from("i am s2 hello");
    let ann = 129;
    let r = function(s1.as_str(),s2.as_str(),ann);
    println!("r = {}",r);
    println!("Hello, world!");
}
