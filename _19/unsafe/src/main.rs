use core::slice;


static HELLO_WORLD: &str = "Hello, world!";

fn main() {
    println!("不安全的RUST");

    println!("\n解引用裸指针");
    let mut num = 5;
    unsafe {
        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;
        println!("num is {num} r1:{} r2:{} ", *r1, *r2); // num is 5 r1:5 r2:5
        num = 6;
        println!("num is {num} r1:{} r2:{} ", *r1, *r2); // num is 6 r1:6 r2:6
    };

    println!("\n调用不安全函数或方法");
    unsafe { dangerous() };

    println!("\n创建不安全代码的安全抽象");
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);
    println!("a is {:?}", a);
    println!("b is {:?}", b);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    println!("\n使用 extern 函数调用外部代码");
    unsafe{
        println!("-3的绝对值是{}", abs(-3));

    }

    println!("\n访问或修改可变静态变量");
    println!("静态变量：{}", HELLO_WORLD);

    
}

extern "C" {
    fn abs(input:i32) ->i32;
}


fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr, mid + 1),
        )
    }
}

unsafe fn dangerous() {
    println!("this is an dangerous function!");
}
