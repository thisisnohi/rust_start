fn main() {
    println!("使用生命周期来确保引用有效");
    scop();
    println!("===结构体定义中的生命周期注解");
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    println!("first_sentence is {first_sentence}");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("i is {:?}", i);
}
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn scop() {
    let r;
    {
        let x = 5;
        // r = &x; // &x的作用域在大括号类
        r = x;
    }
    println!("r:{}", r);

    fn_scop();
}

fn fn_scop() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("result is {result}");

    let string1 = String::from("abcd");
    let mut result = {
        let string2 = "xyZ";
        longest(string1.as_str(), string2)
    };
    println!("result is {result}");
}

fn longest<'a>(p0: &'a str, p1: &'a str) -> &'a str {
    if (p0.len() > p1.len()) {
        return p0;
    } else {
        return p1;
    }
}
