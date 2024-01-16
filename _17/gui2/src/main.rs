use gui2::{Button, DrawTrait, Screen};

fn main() {
    println!("顾及不同类型值的 trait 对象");
    println!("==GUI===");

    // let screen = Screen {
    //     components: vec![
    //         Button {
    //             width: 100,
    //             height: 20,
    //             label: "按钮".to_string(),
    //         },
    //         SelectBox {
    //             width: 100,
    //             height: 20,
    //             options: vec![
    //                 String::from("全部"),
    //                 String::from("第一次"),
    //                 String::from("第二次"),
    //             ],
    //         },
    //     ],
    // };
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 100,
                height: 20,
                options: vec![
                    String::from("全部"),
                    String::from("第一次"),
                    String::from("第二次"),
                ],
            }),
            Box::new(Button {
                width: 100,
                height: 20,
                label: "按钮".to_string(),
            }),
        ],
    };

    // screen.run();
}

// 定义组件
struct SelectBox {
    pub width: i32,
    pub height: i32,
    pub options: Vec<String>,
}

impl DrawTrait for SelectBox {
    fn draw(&self) {
        println!("SelectBox with [{},{}] ", self.width, self.height);
        for item in self.options.iter() {
            println!(" * {item}")
        }
    }
}
