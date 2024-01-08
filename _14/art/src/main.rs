//! # Art
//!
//! A library for modeling artistic concepts.
//! ch14 car使用测试

use nohi_art::{mix, PrimaryColor};

fn main() {
    println!("Hello, world!");
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);

    // add_one 调用
    let num = 10;
    println!("Hello, world! {num} plus one is {}!", add_one::add_one(num));
    println!(
        "Hello, world! {num} plus random is {}!",
        add_one::add_random(num)
    );
}
