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
        println!("mastger name = {}")
    }
}
fn main() {
    println!("Hello, world!");
}


// https://www.bilibili.com/video/BV1FJ411Y71o?t=284.3&p=9