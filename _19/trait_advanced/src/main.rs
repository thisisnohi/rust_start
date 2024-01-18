use std::fmt;
use std::fmt::{Display, Formatter, Pointer};
use std::ops::Add;
use trait_advanced::{Meters, Millimeters};

fn main() {
    println!("高级 trait");
    println!("\n默认泛型类型参数和运算符重载");

    let p1 = Point { x: 1, y: 1 };
    let p2 = Point { x: 1, y: 1 };
    let p4 = Point { x: 2, y: 2 };

    assert_eq!(p1 + p2, p4);

    println!("\nnewtype 模式");
    let meters = Meters { 0: 1 };
    let mill_meters = Millimeters { 0: 1 };
    println!("meters:{:?} mill_meters:{:?}", meters, mill_meters);
    let mill_meters = mill_meters.add(meters);
    println!("mill_meters:{:?}", mill_meters);

    println!("\n完全限定语法与消歧义：调用相同名称的方法");
    let human = Human {};
    human.fly(); // 这个人有特殊功能...飞....
    Pilot::fly(&human); //这是一个飞行员...
    Wizard::fly(&human); // 这是一个哈里波特

    println!("\n完全限定语法");
    println!("A baby dog is called a {}", Dog::baby_name()); // A baby dog is called a Spot

    //  完全限定语法
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name()); // A baby dog is called a puppy

    println!("\n父 trait 用于在另一个 trait 中使用某 trait 的功能");
    let point = PointDisplay { x: 1, y: 3 };
    println!("{}", point);
    println!("===========================");
    crate::OutlinePrint::outline_print(&point);
    println!("===========================");

    println!("\nnewtype 模式用以在外部类型上实现外部 trait");
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}

// newtype 模式用以在外部类型上实现外部 trait
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.join(" , "))
    }
}

trait Animal {
    fn baby_name() -> String;
}
struct Dog;
impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}
impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

trait Pilot {
    fn fly(&self);
}
trait Wizard {
    fn fly(&self);
}
struct Human;
impl Pilot for Human {
    fn fly(&self) {
        println!("这是一个飞行员...")
    }
}
impl Wizard for Human {
    fn fly(&self) {
        println!("这是一个哈里波特");
    }
}

impl Human {
    fn fly(&self) {
        println!("这个人有特殊功能...飞....");
    }
}

#[derive(PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}
impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// 父 trait 用于在另一个 trait 中使用某 trait 的功能
struct PointDisplay {
    x: i32,
    y: i32,
}
pub trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();

        println!("{}", "*".repeat(len + 6));
        println!("* {} *", " ".repeat(len + 2));
        println!("*  {}  *", output);
        println!("* {} *", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 6));
    }
}
impl OutlinePrint for PointDisplay {}

impl fmt::Display for PointDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}
