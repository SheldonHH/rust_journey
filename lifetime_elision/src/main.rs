#[derive(Debug)]
struct A<'a>{
    name:&'a str,
}


fn main() {
    let n = String::from("hello");
    let a = A{name: &n};
    // println!("a = {}",a);  //❌因为没有格式化       cannot be formatted with the default formatter
    println!("a = {:#?}",a);
    println!("Hello, world!");

    let s = get_a_str(&n);
    println!("s = {}",s);
}


fn get_a_str(s: &str) -> &str{
    s
}

/*
🌟🌟🌟三条规则免写, fn定义，impl🌟
1. 一个引用参数的函数，一个生命周期：fn foo<'a> (x: &'a i32)
2. 两个                         fn foo<'a> (x: &'a i32, y: &'b i32)

3. 一个input参数，则赋予所有output生命周期参数
fn foo(x: &i32) -> &i32 
fn foo<'a>(x: &'a i32) -> &'a i32

多个生命周期函数，self的生命周期赋予所有output生命周期参数

*/

