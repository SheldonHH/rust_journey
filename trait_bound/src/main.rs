// 直接
// fn print_information(item: impl GetInformation){
//  trait bound 对参数的要求
// fn print_information<T: GetInformation>(item: T){
//     println!("name = {}", item.get_name());
//     println!("age = {}", item.get_age());
// }


#[derive(Debug)]
pub struct Student{
    pub name: String,
    pub age: u32,
}

trait GetName{
    fn get_name(&self) -> &String;
}
impl GetName for Student{
    fn get_name(&self) -> &String{
        &self.name
    }
}

trait GetAge{
    fn get_age(&self) -> u32;
}
impl GetAge for Student{
    fn get_age(&self) -> u32{
        self.age
    }
}

// 写法 1
// fn print_information<T: GetName+GetAge>(item: T){
//     println!("name = {}", item.get_name());
//     println!("age = {}", item.get_age());
// }

// 写法 2
fn print_information<T>(item: T)
    where T: GetName+GetAge
{
        println!("name = {}", item.get_name());
        println!("age = {}", item.get_age());
}

// 返回值的类型是GetAge特征
fn produce_item_with_age() -> impl GetAge{
    Student{
        name: String::from("xiaoming"),
        age: 15,
    }
}

fn main() {
    println!("Hello, world!");
    let s = Student{name: "Bob".to_string(), age: 10};
    print_information(s);

    let s = produce_item_with_age();
    // println!("{:#?}",s)   // ❌
    // println!("{}",s.name)  // ❌

}