pub trait DrawTrait {
    fn draw(&self);
}

pub struct Screen<T: DrawTrait> {
    pub components: Vec<T>,
}

// 这限制了 Screen 实例必须拥有一个全是 Button 类型或者全是 TextField 类型的组件列表。
// T只有一种,这里T并不是动态的，不像gui中的screen vec<Box<dyn DrawTrait> 为动态类型
impl<T> Screen<T>
where
    T: DrawTrait,
{
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
