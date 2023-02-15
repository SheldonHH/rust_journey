struct StuA<'a>{
    name: &'a str,
}

impl<'b> StuA<'b>{
// impl<'a> StuA<'a>{ //✅
    fn do_sth(&self) -> i32{
        3
    }
     // 🌟 life elision规则三：output lifetime = self或mut self 的lifetime
    fn do_sth2(&self, s: &str) -> &str{
             // 等价于
    // fn do_sth2(&'b self, s: &str) -> &'b str{
        self.name
    }

    // ❌返回并非self
    // fn do_sth3(&self, s: &str) -> &str{
    fn do_sth3<'a>(&'a self, s: &'a str) -> &'a str{
        s
    }
}

fn main() {
    let s = String::from("hello");
    let a = StuA{name: &s};
    println!("{}", a.do_sth());

    let s2 = String::from("hello");
    println!("{}",a.do_sth2(&s2));
    println!("Hello, world!");


    let s3 = String::from("hello");
    println!("{}",a.do_sth3(&s3));
    println!("Hello, world!");
}
