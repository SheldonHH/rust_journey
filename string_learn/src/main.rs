// 1. 空string
// 2. 字面值创建一个String
// 2.1 使用String::from()
// 2.2 使用str方式
// 3. 更新STring
// 3,1 push)str
// 3.2 push
// 3.3 使用"+"
// 3.4 format
// 4. String索引
// 5. str索引 [⚠️以边界]
// 6、遍历
// 6.1 chars
// 6.2 bytes

fn main() {
    let mut s0 = String::new();
    s0.push_str("hellofsd");
    println!("s0 = {}",s0);

    let s1 = String::from("init something");
    println!("{}", s1);

    // 隐藏
    let s1 = "init something".to_string();
    println!("{}", s1);

    let mut s2 = String::from("s2_hello");
    s2.push_str(", world");
    let ss = "ss".to_string();
    s2.push_str(&ss); // 🌟push用引用&

    println!("s2= {}", s2);
    println!("ss= {}", ss);

    let mut s2 = String::from("tea");
    s2.push('m'); // 仅能添加一个字符，且必为单引号
    //  s2.push('mx');
    // s2.push("m"); //双引号需要reference：报错信息：expected char, found reference

    println!("s2= {}", s2);

    // 合并
    let s1 = "hello".to_string();
    let s2 = String::from(", world");
    let s3 = s1 + &s2; // 🌟+用引用&[把s1给s3]
    println!("s3 = {}", s3);

    // println!("s1 = {}", s1); // ❌，s1的使用权已给s3
    println!("s2 = {}", s2); 
    println!("Hello, world!");

    let s341 = String::from("tic");
    let s342 = String::from("tac");
    let s343 = String::from("toe");
    let s344 = format!("{}-{}-{}",s341,s342,s343); //format! = println! 类似
    println!("s344={}",s344);

    // 仍可使用1，2，3
    println!("s341={}",s341);
    println!("s342={}",s342);
    println!("s343={}",s343);


    // 尝试索引
    let s4 = String::from("hello");
    println!("s4.len()={}",s4.len());
    // let s41 = s4[0]; // ❌


    let s5 = String::from("你好");
    println!("s5.len()={}",s5.len());


    let hello = "你好";
    // ✅，可非单个的取, slice方式可以
    let h5 = &hello[0..3]; // 你的边界上0-3
    println!("h5={}",h5);
    
    // let h6 = &hello[0..2]; // thread 'main' panicked at 'byte index 2 is not a char boundary
    // println!("h6={}",h6);
    // println!("Hello world!");

    // chars
    for c in s5.chars(){
        println!("c = {}", c);
    }

    // bytes
    for b in s5.bytes(){
        println!("b = {}", b);
    }

    println!("++++++++++++++++++");
    println!("Hello world");

}
