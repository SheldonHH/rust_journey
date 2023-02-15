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
}
