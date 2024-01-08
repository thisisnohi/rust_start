use std::thread;
use std::time::Duration;

fn main() {
    println!("=============迭代器与闭包=============");

    // 0 初始化库存
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };
    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    println!("\n===迭代器与闭包\n");
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    let rs = expensive_closure(12);
    println!("12 expensive_closure is {rs}");

    let example_closure = |x| x;
    // 下面两行代码只能使用一行，因为上一行代码没有类型，编译器只能通过以下一行代码推断出一个类型使用ß
    // let s = example_closure(String::from("hello"));
    let n = example_closure(5);

    println!("\n===捕获引用或者移动所有权");
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);
    // 这里不能打印，因为borrows_mutably闭包已经借用了list
    // 只能borrows_mutably调用结束后，list被借用结束
    // 才能存在其他的list借用（println! 也是使用的list借用）
    // 注：可变借用存在时不允许有其他借用
    // println!("list is {:?}", list);
    borrows_mutably();
    borrows_mutably();
    borrows_mutably();
    println!("After calling closure: {:?}", list);

    println!("\n===强制闭包获取它用到的环境中值的所有权");
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    println!("\n===将被捕获的值移出闭包和 Fn trait");
    println!("FnMut");
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    // 获取颜色
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
    // 获取库存最多的颜色
    fn most_stocked(&self) -> ShirtColor {
        let mut red_count = 0;
        let mut blue_count = 0;

        for item in &self.shirts {
            match item {
                ShirtColor::Red => red_count += 1,
                ShirtColor::Blue => blue_count += 1,
            }
        }

        if red_count > blue_count {
            return ShirtColor::Red;
        } else {
            return ShirtColor::Blue;
        }
    }
}
