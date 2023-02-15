use add_one;
fn main() {
    let num = 10;
    let r = add_one::add_one(num);
    println!("r === {}", r);
    println!("Hello, world!");
}
/*
cd addr_26
cargo build
cargo run -p adder
*/

