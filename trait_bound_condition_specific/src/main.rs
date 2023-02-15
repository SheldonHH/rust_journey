struct Student{
    name: String,
    // age: u32,
}
trait GetName{
    fn get_name(&self) -> &String;
}
impl GetName for Student{     // 对特定trait实现
    fn get_name(&self) -> &String{
        &(self.name)
    }
}
trait PrintName{
    fn print_name(&self);
}
impl<T:GetName> PrintName for T{ // 对特定trait实现，注意和GetName impl的区别，用PrintName继承GetName
    fn print_name(&self){
        println!("name = {}", self.get_name());
    }
} 


fn main() {
    let s = Student{name: String::from("Yellow")};
    s.print_name();
    println!("Hello, world!");
}
