pub struct Post {
    content: String,
    state: Option<Box<dyn BlogState>>,
}

impl Post {
    pub fn new(content: &str) -> Post {
        Post {
            content: content.to_string(),
            state: Some(Box::new(Draft {})),
        }
    }

    // 设置内容
    pub fn add_text(&mut self, content: &str) {
        self.content.push_str(content);
    }

    pub fn content(&self) -> &str {
        // Option 的 as_ref 方法是因为需要 Option 中值的引用而不是获取其所有权。
        // 因为 state 是一个 Option<Box<dyn State>>，调用 as_ref 会返回一个 Option<&Box<dyn State>>。
        // 如果不调用 as_ref，将会得到一个错误，因为不能将 state 移动出借用的 &self 函数参数。
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        println!("request_review");
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }

    pub fn approve(&mut self) {
        println!("approve");
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }

    pub fn reject(&mut self) {
        println!("reject");
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject());
        }
    }
}

trait BlogState {
    // self: Box<Self> 这个语法意味着该方法只可在持有这个类型的 Box 上被调用。
    // 这个语法获取了 Box<Self> 的所有权使老状态无效化，以便 Post 的状态值可转换为一个新状态。
    fn request_review(self: Box<Self>) -> Box<dyn BlogState>;
    fn approve(self: Box<Self>) -> Box<dyn BlogState>;
    fn reject(self: Box<Self>) -> Box<dyn BlogState>;
    fn content<'a>(&self, post: &'a Post) -> &'a str;
}

struct Draft {}

impl BlogState for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn BlogState> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn BlogState> {
        todo!()
    }

    fn reject(self: Box<Self>) -> Box<dyn BlogState> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct PendingReview {}
impl BlogState for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn BlogState> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn BlogState> {
        Box::new(Published {})
    }
    fn reject(self: Box<Self>) -> Box<dyn BlogState> {
        Box::new(Draft {})
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Published {}
impl BlogState for crate::Published {
    fn request_review(self: Box<Self>) -> Box<dyn BlogState> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn BlogState> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn BlogState> {
        todo!()
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
