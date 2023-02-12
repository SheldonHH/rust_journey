use std::collections::HashMap;
fn main() {
    let mut scores: HashMap<String, i32>= HashMap::new();
    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Red"),20);

    let keys = vec![String::from("Blue"),String::from("Red")];
    let values = vec![10,20];
    let scores: HashMap<_, _> = keys.iter().zip(values.iter()).collect();

    let k = String::from("Blue");
    // if let
    // if let Some(v)= scores.get(&k){ //v=10   🌟OPTION，可能为空
    //     println!("v={}",v);
    // }

    // let k = String::from("Yellow");


    // match
    let v = scores.get(&k);
    match v {
        Some(value) => println!("v = {}", value),
        None => println!("None")
    }
    // 自变量直接创建容器

    println!("+++++++++++++++++++++++++++++++");
    //  遍历:会任意顺序
    for(key, value) in &scores{
        println!("{}, {}", key, value)
    }

    // 直接插入
    let mut ss = HashMap::new();
    ss.insert(String::from("one"),1);
    ss.insert(String::from("one"),321);
    println!("ss={:?}",ss);

    //键不u你存在是
    let mut ss1 =  HashMap::new();
    ss1.insert(String::from("one"),1);
    ss1.entry(String::from("one")).or_insert(3);
    println!("ss1={:?}",ss1);
    println!("ss1={:#?}",ss1);
    

    // 根据旧值，更新一个值
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("map = {:?}",map)
}
