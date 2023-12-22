use std::fmt::{format, Display, Formatter};
use std::sync::atomic::Ordering::SeqCst;

fn main() {
    println!("==5 结构休");

    // 初始化User，允许修改
    let mut user1 = User {
        active: false,
        username: "nohi".to_string(),
        email: "thisisnohi@163.com".to_string(),
        sign_in_count: 0,
    };
    println!("User1: {}", user1);
    user1.username = "NOHI".to_string();
    println!("User1: {}", user1);

    // 通过函数初始化
    let user2 = build_user("thisisnohi".to_string(), "thisisnohi@163.com".to_string());
    println!("User2: {}", user2);

    // 通过忆存在变量初始化，username使用单独的初始化
    let user3 = User {
        username: "nohi".to_string(),
        ..user2
    };
    //println!("User2: {}", user2); // 不能再使用user2,因其已经被user3借用
    println!("User3: {}", user3);

    println!("=======trait==========");

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "Tet rectangle width={} heigth={}, thre area is {}",
        rect1.width,
        rect1.height,
        area_calc(&rect1)
    );

    println!("rect {rect1:#?}");

    dbg!(&rect1);
    println!("rect {rect1:#?}");
    println!("rect area {}", rect1.area());
    println!("rect area {}", Rectangle::area2(&rect1));

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 can hold rect2 {}", rect1.can_hold(&rect2));

    // 关联函数使用
    let rect3 = Rectangle::square(10);
    dbg!(&rect3);
    println!("rect3 {rect3:#?}");
}

fn area_calc(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

#[derive(Debug, PartialEq)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width >= rect.width && self.height >= rect.height
    }
    fn area2(rect: &Rectangle) -> u32 {
        rect.width * rect.height
    }
    // 关联函数
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: false,
        sign_in_count: 0,
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*format!(" username[{}]", self.username));
        f.write_str(&*format!(" email[{}]", self.email));
        f.write_str(&*format!(" active[{}]", self.active));
        f.write_str(&*format!(" username[{}]", self.sign_in_count))
    }
}
