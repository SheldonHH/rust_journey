// 1. 函数错误，错误传给调用者（传播错误 throw error broadcast）
// 2. 传播错误的简写
// 3. 简写
// 4. 何时用panic!，何时用Result
// panic! \unwrap \expect：测试，
// 实际操作用Result throw
// 5. Option 和 Result

use std::io;
use std::io::Read;
use std::fs::File;

fn main() {
    let r = read_username_from_file();
    match r{
        Ok(s) => println!("s = {}", s),
        Err(e) => println!("err = {:?}", e),
    }
    println!("Hello, world!");
}


// 传播错误
// fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("hello.txt");
//     let mut f = match f{
//         Ok(file) => file,
//         Err(error)  => return Err(error),
//     };

//     let mut s = String::new();
//     match f.read_to_string(&mut s){
//         Ok(_) => Ok(s),
//         Err(error) => Err(error),
//     }
// } 


// 传播错误,简写
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut f = File::open("hello.txt")?;

//     let mut s = String::new();
//      //免去match
//     f.read_to_string(&mut s)?;
//     Ok(s)
// } 

// 推荐方式✅
// 传播错误,简简写,
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    // File::open  
    // ::作用域
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)

} 