
fn main() {
    println!("18 模式与模式匹配");
    println!("所有可能会用到模式的位置");

    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using you favorite color, {color}, as the background")
    } else if is_tuesday {
        println!("Tuesday is green day");
    } else if let Ok(age) = age {
        if age > 18 {
            println!("你已长大，要怕绿！");
        } else {
            println!("你还小，好好长大！")
        }
    } else {
        println!("Using blue as the background color");
    }

    println!("\nwhile let 条件循环");
    let mut stack = Vec::new();
    stack.push("1");
    stack.push("3");
    stack.push("3");

    while let Some(str) = stack.pop() {
        println!(" * {str}")
    }

    println!("\nfor 循环");
    let v = vec!["a", "b", "c"];
    println!("stack[{:?}]", v);
    for (index, value) in v.iter().enumerate() {
        println!(" stack[{index}] is [{value}]")
    }

    println!("\nlet 语句");
    let (x, y, z) = (1, 2, 3);
    println!("{x} {y} {z}");
    let (x, ..) = (1, 2, 3);
    println!("{:?}", x);

    println!("\n函数参数");
    let point = (1, 2);
    fn_param_pattern(&point);
    println!("point:{:?}", point);

    println!("\nRefutability（可反驳性）: 模式是否会匹配失效");
    let some_option_value: Option<&str> = None;
    // let Some(x) = some_option_value;
    if let Some(x) = some_option_value {
        println!("some_option_value is null时，永不会走到这里");
    } else {
        println!("这里可以省略");
    }

    println!("\n匹配命名变量");
    let x = Some(10);
    let y = 5;
    match x {
        Some(5) => println!("5"),
        // 此处y不是 上述代码定义为5的y
        // 此处的y匹配任务有效值
        Some(y) => println!("this is y {y}"),
        _ => println!("nothing"),
    }

    println!("\n多个模式");
    let x = 2;
    match x {
        1 | 5 => println!("match 1|5 {x}"),
        _ => println!("nothing {x}"),
    }

    println!("\n通过 ..= 匹配值的范围");
    let c = 'c';
    match c {
        'a'..='z' => println!("小写字母 {c}"),
        'A'..='Z' => println!("小写字母 {c}"),
        _ => println!("非法字母 {c}"),
    }
    println!("\n解构结构体");
    let p = Point { x: 5, y: 10 };
    let Point { y, x } = p;
    println!("point[{x}, {y}]"); // point[5, 10]
    let Point { y: a, x: b } = p;
    println!("point[{x}, {y}]"); // point[5, 10]
    println!("point[{a}, {b}]"); // point[10, 5]

    let p = Point { x: 0, y: 7 };
    match p {
        Point { x, y: 0 } => println!("On the x axis is {:?}", p),
        // 匹配上x后进入
        Point { x: 0, y } => println!("On the y axis is {:?}", p),
        Point { x, y } => println!("Point {:?}", p),
    }

    println!("\n解构枚举");
    let msg = Message::ChangeColor(1, 2, 3);
    match msg {
        Message::Quit => println!("this is quit"),
        Message::Move { x, y } => println!("move {x} {y}"),
        Message::Writer(msg) => println!("writer [{msg}]"),
        Message::ChangeColor(a, b, c) => println!("clor {a} {b} {c}"),
    }

    println!("\n解构嵌套的结构体和枚举");
    let msg = Message2::ChangeColor(Clor::Hsv(1, 2, 3));
    match msg {
        Message2::Quit => println!("quit"),
        Message2::Move { x, y } => println!("move {x} {y}"),
        Message2::Writer(str) => println!("writer [{str}]"),
        Message2::ChangeColor(Clor::Hsv(a, b, c)) => println!("color hsv {a} {b} {c}"),
        Message2::ChangeColor(Clor::Rgb(a, b, c)) => println!("color rgb {a} {b} {c}"),
    }

    println!("\n使用 _ 忽略整个值");
    foo(1, 2);

    println!("\n使用嵌套的 _ 忽略部分值");
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(k_)) => {
            println!("无用功....")
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!(
        "setting_value is {:?}, new_setting_value is {:?}",
        setting_value, new_setting_value
    );

    let numbers = (1, 2, 3, 4, 5);
    match numbers {
        (first, _, third, _, fifty) => {
            println!("first:{first} third:{third} fifty:{fifty}");
        }
    }

    println!("\n通过在名字前以一个 _ 开头来忽略未使用的变量");
    let _x = 5;
    // 编译器会提示，此变量未使用
    let y = 5;

    let _some = Some(5);
    // 这里_some已经移动了
    if let Some(_t) = _some {
        println!("_t is {}", _t);
    }
    // 不能再使用_some了，因为 if let 已经移动了_some
    // println!("unable use _some again {_some}");
    let _some = Some(5);
    // _some没有移动，if let后仍可以使用
    if let Some(_) = _some {
        println!("got the value {:?}", _some);
    }
    println!("unable use _some again {:?}", _some);

    println!("\n用 .. 忽略剩余值");
    let p = Point2 { x: 1, y: 2, z: 3 };

    match p {
        Point2 { x, .. } => {
            println!("x is {x}");
        }
    }

    let numbers = (1, 2, 3, 4, 5);
    match numbers {
        (first, .., last) => {
            println!("first is {first} last is {last}");
        }
    }

    println!("\n匹配守卫提供的额外条件");
    let num = Some(4);
    match num {
        Some(x) if x % 2 == 0 => println!("是偶数"),
        Some(x) => println!("是奇数"),
        _ => println!("不知道是啥"),
    }

    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("match 4|5|5 and y is true"),
        _ => println!("default ..."),
    }

    println!("\n@ 绑定");
    let point = Point2 { x: 3, y: 4, z: 5 };

    match point {
        Point2 {
            // 赋值给变更x_val
            x: x_val @ 1..=2,
            y,
            z,
        } => println!(" x at [1..2] {x} {x_val}"),
        Point2 { x: 3..=8, y, z } => println!(" x at [3..8] {x}"),
        _ => (),
    }
}

struct Point2 {
    x: i32,
    y: i32,
    z: i32,
}

fn foo(_: i32, y: i32) {
    println!("y is {y}");
}

#[derive(Debug)]
enum Clor {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

#[derive(Debug)]
enum Message2 {
    Quit,
    Move { x: i32, y: i32 },
    Writer(String),
    ChangeColor(Clor),
}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Writer(String),
    ChangeColor(i32, i32, i32),
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn fn_param_pattern(&(x, y): &(i32, i32)) {
    println!("location [{x},{y}]");
}
