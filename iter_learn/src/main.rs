// 1. iter 遍历序列中的
// 2 惰性，使用iter前不会有renew效果
// 3. 每一个迭代器都是实现了iterator trait
// trait Iterator{
//     type Item;
//     fn next(mut self) -> Option<Self::Item>; // type Item和Self::Item这种做法叫定义trait的关联类型
// }

// next是Iterator被要爱u实现的唯一方法，next一次返回一个元素
fn main(){
    let v1 = vec![1,2,3];
    // let v1_iter = v1.iter(); // 到目前为止，不会对v1产生任何影响 ❌
    let mut v1_iter = v1.iter(); // 到目前为止，不会对v1产生任何影响 ✅
    // for val in v1_iter{
    //     println!("val = {}",val);
    // }

    // 🌟 if let 方法实现迭代器
    if let Some(v) = v1_iter.next(){
        println!("v = {}", v);
    }else{
        println!("At end");
    }


    // ====迭代可变可变可变可变引用=====
    let mut v2 = vec![1,2,3];
    let mut v2_iter = v2.iter_mut();
    if let Some(v) = v2_iter.next(){
        *v=3;
    }
    println!("v2 = {:?}",v2);


    // ====消费适配器 consuming adaptors=====
    let v3 = vec![1,2,3];
    let v3_iter = v3.iter();
    let total: i32 = v3_iter.sum(); //调用消费适配器sum
    println!("total = {}", total);
 

    // 迭代适配器
    println!("++++++++++++++++++++");
    let v4 = vec![4,4,3];
    let v4_iter = v4.iter();
    println!("v4 = {:?}",v4);
    let v5: Vec<_> = v4.iter().map(|x| x+1).collect();
    println!("v5 = {:?}", v5);
    println!("Hel");

    println!("++++++++++++++++++++");
    let v6 = vec![5,6,7];
    let v6_iter = v6.iter();
    println!("v6 = {:?}",v6);
    let v7: Vec<_> = v6.into_iter().filter(|x| *x>5).collect();
    println!("v7 = {:?}", v7);
    // println!("Hel");



}