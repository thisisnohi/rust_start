use blog11::Post;

fn main() {
    println!("Hello, blog again!");
    let mut post = Post::new("This is NOHI");
    assert_eq!("", post.content());

    post.reject();
    assert_eq!("", post.content());

    // 请求审核
    post.request_review();
    assert_eq!("", post.content());

    post.reject();
    assert_eq!("", post.content());
    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    post.approve();
    assert_eq!("This is NOHI", post.content());
}
