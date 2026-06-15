#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[unsafe(no_mangle)]
fn main() -> i32 {
    println!("========================================");
        println!("  from Li Weiran");
    println!("========================================");

    let a = 100;
    let b = 200;
    let sum = a + b;
    println!("{} + {} = {}", a, b, sum);
    0
}
