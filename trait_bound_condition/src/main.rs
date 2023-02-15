trait GetName{
    fn get_name(&self) -> &String;
}
trait GetAge{
    fn get_age(&self) ->u32;
}

struct PeopleMatchInformation<T,U>{
    master: T,
    student: U
}

impl<T:GetName+GetAge, U:GetName+GetAge> PeopleMatchInformation<T,U>{
    fn print_information(&self){
        println!("mastger name = {}",self.master.get_name());
        println!("mastger name = {}",self.master.get_age());
        println!("student name = {}",self.student.get_name()); 
        println!("student name = {}",self.master.get_age());
    }
}

// 使用
struct Teacher{
    name: String,
    age: u32,
}
impl GetName for Teacher{
    fn get_name(&self) -> &String{
        &(self.name)
    }
}
impl GetAge for Teacher{
    fn get_age(&self) -> u32{
        (self.age)
    }
}

struct Student{
    name: String,
    age: u32,
}
impl GetName for Student{
    fn get_name(&self) -> &String{
        &(self.name)
    }
}
impl GetAge for Student{
    fn get_age(&self) -> u32{
        (self.age)
    }
}


fn main() {
    let s = Student{name: "Karry".to_string(), age: 15};
    let t = Teacher{name: "SGGDS".to_string(), age: 54};
    let m = PeopleMatchInformation{master: t, student: s};
    m.print_information();
    println!("Hello, world!");
}


// https://www.bilibili.com/video/BV1FJ411Y71o?t=284.3&p=9