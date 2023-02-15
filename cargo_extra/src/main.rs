fn main() {
    println!("Hello, world!");
}
// https://www.bilibili.com/video/BV1FJ411Y71o?t=53.8&p=22

/*
 cargo build

 cargo build --release
 cargo run --release


  */

  /*
[package]
name = "cargo_extra"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]

# 编译优化，也是default设定
[profile.dev]
opt-level = 0
[profile.release]
opt-level = 3

cargo build
# O0 in 0.25s
# O3 in 0.28s
  */

