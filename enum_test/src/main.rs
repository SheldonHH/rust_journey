enum Option<T>{ // T代表范型
    Some(T),
    None,
}

std::option::Option
// 2。使用方式
fn main() {
    let some_number = Some(5);
    let some_striong = Some(String::from("a string"));
    let absent_number: Option<i32> = None;

    let x: i32 = 5;
    let y: Option<i32> = Some(5);

    // 加法, 错误
    // let sum = x+y; 
    let mut temp = 0;     // 如何使用y的值
    match y {
        Some(i) => { temp = i; }
        None => {println!("do nothing");}
    }
    let sum = x+temp;
    println!("sum = {}",sum);

    // 表示空的
    println!("Hello, world!");
}
