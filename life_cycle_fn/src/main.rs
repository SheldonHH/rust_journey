// fn longest(x: &str, y:&str) -> &str{ //expected lifetime parameter  // ❌
// fn longest(x: &'a str, y:&'a str) -> &'a str{  //❌
// fn longest<'a>(x: &'a str, y:&'a str) -> &'a str{ 
// x，y的生命周期不能小于str的生命周期，保证返回值不会变成悬垂引用
fn longest<'c>(x: &'c str, y:&'c str) -> &'c str{  // 字母无所谓
    if x.len() > y.len(){
        x
    }else{
        y
    }
}
// x，y的生命周期不能小于str的生命周期，保证返回值不会变成悬垂引用
// fn get_str(x: &'a str, y:&'a str) -> &'a str{ // ❌
fn get_str<'a>(x: &'a str, y:&'a str) -> &'a str{
    x
}


// 🌟 String在堆上创建，返回的是r的reference，但此时内存已经回收
// 创建并返回
// fn create_r(x: &'a str, y: &'a str) -> &'a str{ // ❌
/*
fn create_str<'a>(x: &'a str, y: &'a str) -> &'a str{ 
    let r = String::from("创建新");
    // r.as_str(); // ❌ help: remove this semicolon to return this value
    r.as_str() //❌❌ ^^ returns a reference to data owned by the current function
}
 */

fn main() {
    let s1 = String::from("abcde");
    let s2 = String::from("ab");
    let r = longest(s1.as_str(),s2.as_str());
    println!("Hello, world!");

    let ss = get_str(s1.as_str(),s2.as_str());
    let new22 = create_str(s1.as_str(), s2.as_str());
}
