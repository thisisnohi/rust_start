pub trait Draw {
    fn draw(&self);
}
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Button[{}][{}-{}]", self.label, self.width, self.height);
    }
}

// 列表平均值
#[derive(Debug)]
pub struct AveragedCollection {
    vec: Vec<i32>,
    // 平均值
    pub average: f32,
}

impl AveragedCollection {
    pub fn build() -> AveragedCollection {
        AveragedCollection {
            vec: vec![],
            average: 0 as f32,
        }
    }
    fn update_average(&mut self) {
        let sum: i32 = self.vec.iter().sum();
        let count = self.vec.len();
        println!("sum/count {}/{}", sum, count);
        self.average = sum as f32 / count as f32;
    }

    pub fn add(&mut self, val: &i32) {
        self.vec.push(*val);
        self.update_average();
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.vec.pop()
    }
}
