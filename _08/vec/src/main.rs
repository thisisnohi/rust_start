fn main() {
    println!("Hello, world!");

    // 没有初始值，需要指定一个类型
    let v: Vec<i32> = Vec::new();

    println!("v1 {:?}", v);

    // vec! 宏，初始化vec
    let mut v = vec![1, 2, 3];
    println!("v2 {:?}", v);
    v.push(4);
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("v3 {:?}", v);

    println!("===>读取vector");
    let mut v0 = v[0];
    println!("v0:{v0}");
    let v2 = v[2];
    println!("v2:{v2}");

    v.push(9);
    v0 = 11;
    println!("v0:{v0}");
    println!("v[0]:{}", v[0]);

    let v3 = v.get(3);
    println!("v3:{:?}", v3);

    //let v100 = v[100];
    let v100 = v.get(100);
    println!("v100:{:?}", v100);

    // 直接修改元素
    println!("直接修改元素");
    let mut v = vec![1, 2, 3];
    let mut v0: &i32 = &v[0];
    println!("v0:{:?}", v0);
    v0 = &10;
    println!("v0:{:?}", v0);
    v.push(4);
    println!("v:{:?}", v);

    // 遍历vector
    bianli_vector();

    // vector 使用枚举存放多种类型的值
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("v:{:?}", row);
}
#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn bianli_vector() {
    println!("===>遍历vector");
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    // 修改vet里的每一个元素
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("v: {:?}", v);
}
