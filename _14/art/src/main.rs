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
}
