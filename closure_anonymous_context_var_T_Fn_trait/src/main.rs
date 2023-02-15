// 1. 匿名函数，允许捕获调用者作用域的值
// 2. 使用方式
// 3. 使用泛型和Fn trait的闭包
fn main() {
    let use_closure = ||{ // || 传参
        println!("This is a closure");
    };
    use_closure();
    println!("Hello, world!");



    let add_one_v2 = |x: u32| ->u32{
        x+1
    };
    // 闭包定义和每一个param和返回值推到具体类型，但无法推导两次🌹
    let add_one_v3 = |x| {x+1};
    let add_one_v4 = |x| x+1;
    let a = add_one_v1(5); //使用函数fn
    let b = add_one_v2(5);
    let c = add_one_v3(5);
    let d = add_one_v4(5);

    println!("a= {}, b={}, c={}, d={}", a,b,c,d);

    // 但无法推导两次🌹例子：
    let example_closure = |x| x;
    let s = example_closure(String::from("hello")); //首次推导，编译器记住了是string类型💡
    println!("s={}",s); 
    // let n = example_closure(5); //🌹    expected struct `String`, found integer
    let n = example_closure(5.to_string()); 
    println!("n={}",n);


    // 捕获环境中的变量
    let i = 1;
    let exe = |x| x+i;
    let r = exe(5);
    println!("r = {}",r);

}

/*
语法格式 🌟fn}无需; 但closure需要;🌟
*/

fn add_one_v1(x: u32) -> u32{
    x + 1
}

