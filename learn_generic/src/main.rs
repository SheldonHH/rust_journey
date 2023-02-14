// 对泛型的约束， 具有Copy特征
// fn largest<T: PartialOrd + Copy>(list: &[T]) -> T{
//     let mut larger = list[0];
//     for &item in list.iter(){
//         if item > larger {
//             larger = item;
//         }
//     }
//     larger
// } 

// fn main() {
//     println!("Hello, world!");

//     let num_list = vec![1,2,23,34,8,100];
//     let max_num = largest(&num_list);
//     println!("max_num = {}",max_num);


//     let char_list = vec!['a','y','b'];
//     let max_char = largest(&char_list);
//     println!("max_char = {}", max_char);


// }


// ------------------------------------结构体中用泛型
#[derive(Debug)] //打印结构体 {#?}
struct Point<T>{
    x:T,
    y:T,
}

fn main(){
    let inteer = Point{x: 1, y:2};
    println!("{:#?}", inteer);

    let inteer = Point{x: 0.32, y:0.02};
    println!("{:#?}", inteer);
}