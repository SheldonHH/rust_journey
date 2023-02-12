// #[derive(Debug)]
fn main() {
    let some_num = Some(5);
    let some_string = Some(String::from("a string"));

    let absent_number: Option<i32> = None;

    let x: i32 = 5;
    let y: Option<i32> = Some(5);
    let mut temp = 0;

    match y{
        Some(i) => {temp = i;}
        None => { println!("do nothing"); }
    }
    let sum = x + temp; 
    println!("sum = {}",sum);


    // let result = plus_one(y);
    // match result {
    //     Some(i) => println!("result = {}", i),
    //     None => println!("nothing")
    // };

    if let Some(value) = plus_one(y){
        println!("value = {}", value);
    } else {
        println!("do nothing");
    }


    println!("Hello, world!");
}


// 函数里面使用
fn plus_one(x: Option<i32>) -> Option<i32> {
    //NOTE: match一定要处理完所有情况
    match x {
        None => None,
        Some(x) => Some(x+1),
    }
}