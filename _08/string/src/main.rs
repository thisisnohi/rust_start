fn main() {
    println!("Hello, world!");

    let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string();
    // 该方法也可直接用于字符串字面值：
    let s = "initial contents".to_string();
    let s = String::from("initial contents");

    let hello = String::from("السلام عليكم");
    println!("Hello {hello}");
    let hello = String::from("Dobrý den");
    println!("Hello {hello}");
    let hello = String::from("Hello");
    println!("Hello {hello}");
    let hello = String::from("שָׁלוֹם");
    println!("Hello {hello}");
    let hello = String::from("नमस्ते");
    println!("Hello {hello}");
    let hello = String::from("こんにちは");
    println!("Hello {hello}");
    let hello = String::from("안녕하세요");
    println!("Hello {hello}");
    let hello = String::from("你好");
    println!("Hello {hello}");
    let hello = String::from("Olá");
    println!("Hello {hello}");
    let hello = String::from("Здравствуйте");
    println!("Hello {hello}");
    let hello = String::from("Hola");
    println!("Hello {hello}");

    println!("===更新字符串");
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("s is {s}");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 is {s1}");
    println!("s2 is {s2}");

    println!("====使用 + 运算符或 format! 宏拼接字符串====");
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用

    // println!("s1 is {s1}"); // s1被借走了，无法使用
    println!("s2 is {s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("s is {s}");

    println!("===索引字符串");
    let hello = "Здравствуйте";
    // let answer = &hello[0];

    println!("===字符串 slice");
    let hello = "Здравствуйте";
    let s = &hello[0..4]; // s 将会是 “Зд”
    println!("s is {s}");
    // let s = &hello[0..1]; // 异常

    println!("===遍历字符串的方法");
    // 打印char
    for c in hello.chars() {
        println!("{c}")
    }
    // 打印字节
    for b in "Зд".bytes() {
        println!("{b}");
    }
}

fn getPri(s: &str) {
    println!("get Pri s is {s}")
}
