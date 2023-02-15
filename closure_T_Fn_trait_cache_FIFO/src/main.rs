//实现缓存，仅处理首次传入值并保存
struct Cacher<T>
    where T:Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32{
        fn new(calculation: T) -> Cacher<T>{
            // 🌟 传入参数和字段名同名 可省略🌟
            Cacher {
                calculation,
                value: None
            }
        }
        // 因为value有可能空，则需要使用match
        fn value(&mut self, arg: u32) -> u32{
            match self.value{
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg); // 设为传入参数
                    self.value = Some(v);
                    v
                }
            }
        }
    }
fn main() {
    // 1. 创建缓存
    // let mut c = Cacher.new(|x| x+1); // ❌
    let mut c = Cacher::new(|x| x+1); //
    let v1 = c.value(1);
    println!("v1={}",v1);

    let v2 = c.value(2);
    println!("v2={}",v2);
    println!("Hello, world!");
}
