use traitoop::{AveragedCollection, Button, Draw, Screen};

fn main() {
    println!("求平均值");
    let mut avg = AveragedCollection::build();
    avg.add(&1);
    avg.add(&2);
    avg.add(&3);
    // avg.add(&4);
    println!("avg:{:?}", avg);

    println!("顾及不同类型值的 trait 对象");

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}

pub struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for crate::SelectBox {
    fn draw(&self) {
        println!("SelectBox[{}-{}]", self.width, self.height);
        for item in self.options.iter() {
            println!(" * {item}")
        }
    }
}
