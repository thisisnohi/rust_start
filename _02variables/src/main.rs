fn main() {
    println!("Hello, world!");

    let x = 5;
    println!("x is {x}");
    // x = 6; // 此处异常

    let mut x = 5;
    println!("x is {x}");
    x = 6; // 此处正常
    println!("x is {x}");

    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup is: {} {} {}", tup.0, tup.1, tup.2);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // 所有权
    let s = String::from("hello"); // s 进入作用域
    println!("s : {s}");
    takes_ownership(s); // s 的值移动到函数里 ...
                        // ... 所以到这里不再有效
                        // println!("s : {s}"); // 此处编译报错

    let x = 5; // x 进入作用域

    makes_copy(x); // x 应该移动函数里，
                   // 但 i32 是 Copy 的，
                   // 所以在后面可继续使用 x
}
// 这里，x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
// 没有特殊之处

fn takes_ownership(some_string: String) {
    // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。
  // 占用的内存被释放

fn makes_copy(some_integer: i32) {
    // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。没有特殊之处
