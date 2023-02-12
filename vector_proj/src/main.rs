// 1. 创建空vector, Vec<T>
// 2. 创建包含初始值的vector
// 3. 丢弃vecotr
// 4. 读取元素
// 5. 更新
// 6. 遍历
// 7. 枚举
fn main() {
    // 1. 不可变
    // let v: Vec<i32> = Vec::new();
    let mut v: Vec<i32> = Vec::new();
    
    v.push(1);

    // 2. 
    let v = vec![1,2,3];

    // 3. 作用域
    {
        let v1 = vec![1,2,3];
    }

    // 4. 下标索引读取
    let one: &i32 = &v[0];
    // let four: &i32 = v[3];
    // println!("one = {}", one) // ❌
    println!("one = {}", *one); // ✅


    // (2) 推荐的方法（match内部一定要用逗号，而非分号）
    match v.get(1){
        Some(value) => println!("value = {}",value),
        _ => println!("None!"),

    }
    // 5. 更新
    let mut v2: Vec<i32> = Vec::new();
    v2.push(1);
    v2.push(2);
    v2.push(3);


    // 6. read（不可变）；可变的遍历
    for i in &v2{
        println!("i = {}",i);
    }
    // 可变的遍历
    for i in &mut v2{
        *i += 1;
        println!("i = {}",i)
    }


    //7. 枚举
    enum Context{
        Text(String),
        Float(f32),
        Int(i32),
    };

    // 放入vector里面
    let c = vec![
        Context::Text(String::from("string")),
        Context::Int(-1),
        Context::Float(0.001)
    ];

    //8. 小知识点
    let mut v = vec![1,2,3,4,5];
    let first = &v[0]; // non-mut reference
    v.push(6); //mut reference
    println!("first = {}", first); //此时不可使用first

    println!("Hello, world!");



    
}
