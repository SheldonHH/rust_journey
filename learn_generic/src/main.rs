// 对泛型的约束， 具有Copy特征
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T{
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
