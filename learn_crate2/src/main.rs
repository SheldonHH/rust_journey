// cargo new --lib mylib
pub mod factory;
fn main() {
    factory::produce_refrigerator::produce_re();
    println!("Hello, world!");
}
 