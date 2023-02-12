#[derive(Debug)]
struct Dog {
    name: String,
    weight: f32,
    height: f32,
}

 // 返回名字的引用，防止修改
impl Dog{
    fn get_name(&self) -> & str{
        &(self.name[..])
    }

    fn get_weight(&self) -> f32{
        self.weight
    }

    fn get_height(&self) -> f32{
        self.height
    }

    fn show() {
        println!("oh oh oh");
        // println!("name = {}", dog.get_name());
        // println!("weight = {}", dog.get_weight());
    }
}

fn main() {
    let dog = Dog{
        name: String::from("wangcai"),
        weight: 100.54,
        height: 70.0,
    };
    // #? 换行
    println!("dog = {:#?}", dog);
    println!("dog = {:#?}", dog.get_height());
    println!("Hello, world!");

    Dog::show();
}
