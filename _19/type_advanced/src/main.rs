fn main() {
    println!("高级类型");
    println!("\n类型别名用来创建类型同义词");
    type KiloMeters = i32;
    let x = 5;
    let y: KiloMeters = 5;
    println!("x = {}, y = {}  x+y={}", x, y, x + y);

    let f: Thunk = Box::new(|| println!("hi"));
    f();
    takes_long_type(f);
    let rs = return_long_type("123");
    rs();

    println!("\n从不返回的 never type");
    println!("\n动态大小类型和 Sized trait");
    // let s: str = *"111";
    // println!("{}", s);
}

type Thunk = Box<dyn Fn() + Send + 'static>;
fn takes_long_type(f: Thunk) {
    f();
}
fn return_long_type(str: &str) -> Thunk {
    Box::new(|| println!("return_long_type"))
}
