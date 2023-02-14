// 1. 共享功能，= interface
pub trait GetInformation{
    fn get_name(&self) -> &String;
    fn get_age(&self) -> u32;
}

// 3. 实现特征
pub struct Student {
    pub name: String,
    pub age: u32,
}

impl GetInformation for Student{
    fn get_name(&self) -> &String{
        &self.name
    }
    fn get_age(&self) -> u32{
        self.age
    }
}

pub struct Teacher {
    pub name: String,
    pub age: u32,
    pub subject: String,
}

impl GetInformation for Teacher{
    fn get_name(&self) -> &String{
        &self.name
    }
    fn get_age(&self) -> u32{ 
        self.age
    }
}

trait SchoolName{
    // drop掉，若使用&String会造车悬垂引用， 因为创建的String再堆上会被drop https://www.bilibili.com/video/BV1FJ411Y71o?t=184.5&p=5
    fn get_school_name(&self) -> String{
        String::from("Victoria")
    } // 外面会造成悬垂引用
}
// 4. 默认实现
impl SchoolName for Teacher{}
// impl SchoolName for Student{}
// 4. 非默认实现
impl SchoolName for Student{
    fn get_school_name(&self) -> String{
        String::from("RJC")
    } // 外面会造成悬垂引用
}

 // 5. trait作为参数
fn print_information(item: impl GetInformation){
    println!("name = {}", item.get_name());
    println!("age = {}", item.get_age());
}


fn main() {
    let s = Student{name: "Xiaoming".to_string(), age: 10};
    let t = Teacher{name: "Dr. Liu".to_string(), age: 50, subject: String::from("math")};

    let s_school_name = s.get_school_name();
    println!("s_school_name = {}",s_school_name);
    let t_school_name = t.get_school_name();
    println!("t_school_name = {}",t_school_name);
    // let t_school_name
    // println!("s, name = {}, age={}", s.get_name(), s.get_age());
    // // s.get_sc
    // println!("t, name = {}, age={}", t.get_name(), t.get_age());
    println!("Hello, world!");
}
