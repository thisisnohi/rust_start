use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;
#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    println!("宏");
    println!("\n使用 macro_rules! 的声明宏用于通用元编程");
    let v: Vec<i32> = vec![1, 2, 3];

    println!("\n如何编写自定义 derive 宏");
    Pancakes::hello_macro();
}
