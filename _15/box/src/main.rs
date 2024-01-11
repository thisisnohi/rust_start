use crate::List::{Cons, Nil};
use crate::List2::Cons2;
use std::ops::Deref;
use std::rc::Rc;

fn main() {
    println!("使用Box<T>指向堆上数据");
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("list is {:?}", list);

    println!("追踪指针的值");
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(x, *y);
    // 不能比较5 与 y,因为为y为引用，需要*y 解引用后获取值比较
    //assert_eq!(5, y);
    println!("像引用一样使用 Box<T>");
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    println!("自定义智能指针");
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    // impl<T> Deref for MyBox<T> 后可以使用*y
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    let m = MyBox::new("Rust");
    hello(&m);

    println!("\n使用 Drop Trait 运行清理代码");
    let a = CustomerSmartPointer {
        data: "aaaa".to_string(),
    };
    CustomerSmartPointer {
        data: "bbb".to_string(),
    };
    let c = CustomerSmartPointer {
        data: String::from("cccc stuff"),
    };
    drop(c);
    let d = CustomerSmartPointer {
        data: String::from("dddd stuff"),
    };

    println!("a is {:?}", a);

    println!("\n使用 Rc<T> 共享数据");
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a)); // 创建b时，a已经被b所拥有，这里不能再使用a

    let a = Rc::new(Cons2(5, Rc::new(Cons2(10, Rc::new(List2::Nil)))));
    let b = Cons2(3, Rc::clone(&a));
    let c = Cons2(4, Rc::clone(&a));

    println!("\n克隆 Rc<T> 会增加引用计数");
    let a = Rc::new(Cons2(5, Rc::new(Cons2(10, Rc::new(List2::Nil)))));
    println!("Count after create a = {}", Rc::strong_count(&a));
    let b = Cons2(3, Rc::clone(&a));
    println!("Count after create b = {}", Rc::strong_count(&a));
    {
        let c = Cons2(3, Rc::clone(&a));
        println!("Count after create c = {}", Rc::strong_count(&a));
    }
    println!("Count after goes out of scope= {}", Rc::strong_count(&a));
}

#[derive(Debug)]
struct CustomerSmartPointer {
    data: String,
}
impl Drop for CustomerSmartPointer {
    fn drop(&mut self) {
        println!("CustomerSmartPointer...drop {}", self.data);
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(t: T) -> MyBox<T> {
        MyBox(t)
    }
}
// 实例解引用
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {}

#[derive(Debug)]
enum List2 {
    Cons2(i32, Rc<crate::List2>),
    Nil,
}
impl List2 {}
