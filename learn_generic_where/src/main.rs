// 对泛型的约束， 具有Copy特征
// https://www.bilibili.com/video/BV1FJ411Y71o?p=8
// fn largest<T: PartialOrd + Copy>(list: &[T]) -> T{
fn largest<T>(list: &[T]) -> T
    where T:  PartialOrd + Copy 
{
    let mut larger = list[0];
    for &item in list.iter(){
        if item > larger {
            larger = item;
        }
    }
    larger
} 

fn main() {
    println!("Hello, world!");

    let num_list = vec![1,2,23,34,8,100];
    let max_num = largest(&num_list);
    println!("max_num = {}",max_num);


    let char_list = vec!['a','y','b'];
    let max_char = largest(&char_list);
    println!("max_char = {}", max_char);


}


// ------------------------------------结构体中用泛型/ ------------------------------------
// #[derive(Debug)] //打印结构体 {#?}
// struct Point<T>{
//     x:T,
//     y:T,
// }

// struct Point2<T,U>{
//     x:T,
//     y:U,
// }


// fn main(){
//     let inteer = Point{x: 1, y:2};
//     println!("{:#?}", inteer);

//     let float = Point{x: 0.32, y:0.02};
//     println!("换行{:#?}", float);
//     println!("不换行{:?}", float);

//     let a = Point2{x: 1.1,y:'a'};
//     println!("a={:?}",a);
// }


// ------------------------------------enum枚举中用泛型/ ------------------------------------
enum Option<T>{
    Some(T),
    None,
}

enum Result<T,E>{
    Ok(T),
    Err(E)
}

// ------------------------------------impl<T> Point<T>中用泛型/ ------------------------------------
// #[derive(Debug)] //打印结构体 {#?}
// struct Point<T>{
//     x:T,
//     y:T,
// }
// impl<T> Point<T>{
//     fn get_x(&self) -> &T{
//         &self.x
//     }
//     fn get_y(&self) -> &T{
//         &self.y
//     }
  
// }


// struct Point2<T,U>{
//     x:T,
//     y:U,
// }
// impl<T,U> Point2<T,U>{
//     fn creat_point<V,W>(self, other: Point2<V,W>) -> Point2<T,W>{
//         Point2 {
//             x: self.x,
//             y: other.y,
//         }
//     }
// }
// // compile时，对泛型代码单态化保证效率
// fn main(){
//     let p = Point{x:1, y:2};
//     println!("x = {}", p.get_x());
//     println!("y = {}", p.get_y());

//     let p = Point{x:0.0321, y:0.002};
//     println!("x = {}", p.get_x());
//     println!("y = {}", p.get_y());

//     println!("+++++++++++++++++++++++");
//     let p1 = Point2{x:5, y:1.1};
//     let p2 = Point2{x:"hello",y:'c'};
//     let p3 = p1.creat_point(p2);
//     println!("p3.x={}, p3.y={}",p3.x, p3.y);
// }