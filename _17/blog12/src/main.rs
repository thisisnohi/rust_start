use blog12::Post;

fn main() {
    println!("Hello, blog again!");
    let mut draft = Post::new();
    draft.add_text("this is text");

    let review = draft.request_review();
    let apprvoe = review.approve();
    println!("apprvoe.content():{}", apprvoe.content())
}
