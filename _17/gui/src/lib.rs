pub trait DrawTrait {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn DrawTrait>>,
}

impl Screen {
    pub fn run(&self) {
        for item in self.components.iter() {
            item.draw();
        }
    }
}

// 定义组件
pub struct Button {
    pub width: i32,
    pub height: i32,
    pub label: String,
}

impl DrawTrait for Button {
    fn draw(&self) {
        println!("Button with [{},{}]", self.width, self.height);
    }
}
