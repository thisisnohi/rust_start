use std::collections::HashMap;
use std::fmt::format;
use std::ops::AddAssign;

fn main() {
    println!("Hello, world!");

    println!("============中位数、众数====================");
    // 给定一系列数字，使用 vector 并返回这个列表的中位数（排列数组后位于中间的值）和众数（mode，出现次数最多的值；这里哈希 map 会很有帮助）。
    // 获取中位数
    let mut num_vec = vec![1, 8, 10, 2, 3, 5, 2, 2, 2, 46];

    let mid_num = get_mid_num(&num_vec);
    println!("中位数:{mid_num:?}");
    match mid_num {
        Some(x) => println!("中位数 = {x}"),
        _ => (),
    }
    // 获取众数
    let mode_num = get_mode_num(&num_vec);
    println!("众数:{:?}", mode_num);
    match mode_num {
        Some(x) => println!("众数 = {x}"),
        _ => (),
    }
    let mode_num = find_mode(&num_vec);
    println!("众数:{:?}", mode_num);
    match mode_num {
        Some(x) => println!("众数 = {x}"),
        _ => (),
    }
    println!("第一次优化");
    let mode_num = find_mode2(&num_vec);
    println!("众数:{:?}", mode_num);
    match mode_num {
        Some(x) => println!("众数 = {x}"),
        _ => (),
    }
    println!("第二次优化");
    let mode_num = find_mode3(&num_vec);
    println!("众数:{:?}", mode_num);
    match mode_num {
        Some(x) => println!("众数 = {x}"),
        _ => (),
    }

    println!("============Pig Latin====================");
    // 将字符串转换为 Pig Latin，
    // 也就是每一个单词的第一个辅音字母被移动到单词的结尾并增加 “ay”，所以 “first” 会变成 “irst-fay”。
    // 元音字母开头的单词则在结尾增加 “hay”（“apple” 会变成 “apple-hay”）。牢记 UTF-8 编码！
    // 元音字母有 a、e、i、o、u 五个。
    // 辅音分别是：b、c、d、f、g、h、j、k、l、m、n、p、q、r、s、t、v、w、x、y、z。

    let mut str = "first";
    // str = "中国";
    let mut rs = pig_latin(str);
    println!("str[{str}] pig latin is [{rs}]");
    str = "hay";
    rs = pig_latin(str);
    println!("str[{str}] pig latin is [{rs}]");
    str = "apple";
    rs = pig_latin(str);
    println!("str[{str}] pig latin is [{rs}]");
    println!("============Pig Latin 多单词版本====================");
    let mut words = "Hello.     apple";
    println!("{} ==> {}", words, pig_latin2(&words.to_string()));
    words = "你好  中国";
    println!("{} ==> {}", words, pig_latin2(&words.to_string()));

    /*
     * 使用哈希 map 和 vector，创建一个文本接口来允许用户向公司的部门中增加员工的名字。
     * 例如，“Add Sally to Engineering” 或 “Add Amir to Sales”。
     * 接着让用户获取一个部门的所有员工的列表，或者公司每个部门的所有员工按照字典序排列的列表。
     */
    println!("============员工和部门====================");
    let mut dept_map: HashMap<String, Vec<String>> = HashMap::new();
    add_use(&mut dept_map, "Computer".to_string(), "NOHI".to_string());
    add_use(&mut dept_map, "Computer".to_string(), "NOHI".to_string());
    add_use(&mut dept_map, "Computer".to_string(), "Mandy".to_string());
    add_use(&mut dept_map, "Computer".to_string(), "Mandy".to_string());
    add_use2(&mut dept_map, "Computer".to_string(), "第一".to_string());
    add_use2(&mut dept_map, "Computer".to_string(), "第二".to_string());
    add_use2(&mut dept_map, "Computer".to_string(), "A".to_string());

    println!("dept_map[{:?}]", dept_map);

    //get_dept_users(&deptMap);
    let dept = "Computer".to_string();
    let dept_users = get_dept_users(&mut dept_map, &dept);
    println!("{} users {:?}", dept, dept_users);
    println!("{} users {:?}", dept, get_dept_users2(dept_map, &dept));
}

fn get_dept_users(dept_map: &mut HashMap<String, Vec<String>>, dept: &String) -> Vec<String> {
    let mut vec: Vec<String> = vec![];
    // let mut dept_user = dept_map.get(dept);
    // let deptUser = dept_user.get_or_insert(&vec);
    if let Some(x) = dept_map.get_mut(dept) {
        x.sort();
        vec = x.clone();
    } else {
        println!("no dept")
    }

    vec
}

// 优化后
fn get_dept_users2(dept_map: HashMap<String, Vec<String>>, dept: &String) -> Vec<String> {
    // 拷贝一份数据，不修改原数据
    let mut dept_map = dept_map.clone();
    let mut vec = vec![];
    dept_map.get_mut(dept).get_or_insert(&mut vec).to_vec()
}

fn add_use(dept_map: &mut HashMap<String, Vec<String>>, dept: String, user: String) {
    println!("Add {user} to {dept}");

    if let Some(x) = dept_map.get_mut(&dept) {
        x.push(user);
    } else {
        let vec: Vec<String> = vec![user];
        dept_map.insert(dept, vec);
    }
}

fn add_use2(dept_map: &mut HashMap<String, Vec<String>>, dept: String, user: String) {
    let user_name = user.clone();
    dept_map
        .get_mut(&dept)
        .get_or_insert(&mut vec![])
        .push(user);
    //
    dept_map.entry(dept).or_insert(Vec::new()).push(user_name);
}
/**
Pig Latin
*/
fn pig_latin(str: &str) -> String {
    println!("str {str}");

    let mut chars = str.chars();
    let mut first_char = chars.next().unwrap();

    let mut tmp_str = "".to_string();
    for char in chars {
        tmp_str = tmp_str + &*String::from(char);
    }
    println!("first_char {first_char}");
    match first_char {
        'a' | 'e' | 'i' | 'o' | 'u' => tmp_str = String::from(str) + "-hay",
        _ => {
            println!("====辅音====");
            tmp_str = tmp_str + "-" + &*String::from(first_char) + "ay";
        }
    }

    tmp_str
}

// 根据benny huo 视频修改
// 多单词句子处理
fn pig_latin2(str: &String) -> String {
    str.split_whitespace().fold(String::new(), |mut acc, word| {
        println!("===word[{}]", word);
        let mut chars = word.chars();
        // 第一个char
        let first_char = chars.next().unwrap();
        // 替换
        let rs = match first_char {
            'a' | 'e' | 'i' | 'o' | 'u' => String::from(word) + "-hay ",
            _ => {
                //tmp_str = tmp_str + "-" + &*String::from(first_char) + "ay";
                format!("{}-{first_char}ay ", chars.as_str())
            }
        };
        acc.push_str(&rs);

        acc
    })
}

/*
* 获取众数
*/
fn get_mode_num(vec: &Vec<i32>) -> Option<i32> {
    if vec.is_empty() {
        return Option::None;
    }

    // 最大值
    let mut max = 0;
    // 最大值索引
    let mut max_key = *vec.get(0).get_or_insert(&0);

    let mut map: HashMap<i32, i32> = HashMap::new();
    // 统计次数
    for i in vec.iter() {
        if let Some(x) = map.get_mut(i) {
            *x += 1;
        } else {
            map.insert(*i, 1);
        }
    }

    for x in map.iter() {
        let key = x.0;
        let value = x.1;
        if value >= &max {
            max_key = key;
            max = *value;
        }
    }

    println!("Max Count is {max_key} count {max}");

    Option::Some(*max_key)
}

// 根据benny huo 视频修改
fn find_mode(vec: &Vec<i32>) -> Option<i32> {
    if vec.is_empty() {
        return Option::None;
    }
    // 统计次数
    let mut map: HashMap<&i32, i32> = HashMap::new();

    for x in vec {
        // let mut count = map.get_mut(x).get_or_insert(0);
        let mut count = map.entry(&x).or_insert(0);
        *count += 1;
    }

    let mut max_key = 0;
    let mut max_value = 0;
    for (k, v) in map.iter() {
        if max_value < *v {
            max_key = **k;
            max_value = *v;
        }
    }
    println!("Number[{max_key}] have max count[{max_value}]");

    Option::Some(max_key)
}

// 根据benny huo 视频修改
// 第一次优化后
fn find_mode2(vec: &Vec<i32>) -> Option<i32> {
    vec.iter()
        // 折叠
        .fold(HashMap::new(), |mut acc, a| {
            // let mut count = acc.entry(a).or_insert(0);
            // *count += 1;
            acc.entry(a).or_insert(0).add_assign(1);
            acc
        })
        .iter()
        .reduce(|acc, e| {
            if acc.1 > e.1 {
                return acc;
            } else {
                return e;
            }
        })
        .map(|item| **item.0)

    // Option::None
}

// 根据benny huo 视频修改
// 第二次优化后
fn find_mode3(vec: &Vec<i32>) -> Option<i32> {
    // 不稳定版本存在此函数
    // vec.group_by(|i,j| i==j).map(|e|(e.first().unwrap(), e.len()))
    // .max_by(|a,b|(*a).1.cmp(&(*b).1)))
    // .map(|e| *e.0)

    vec.iter()
        // 折叠
        .fold(HashMap::new(), |mut acc, a| {
            // let mut count = acc.entry(a).or_insert(0);
            // *count += 1;
            acc.entry(a).or_insert(0).add_assign(1);
            acc
        })
        .iter()
        .max_by(|a, b| a.1.cmp(b.1))
        .map(|item| **item.0)
}

// 给定一系列数字，使用 vector 并返回这个列表的中位数（排列数组后位于中间的值）和众数（mode，出现次数最多的值；这里哈希 map 会很有帮助）。
fn get_mid_num(vec: &Vec<i32>) -> Option<i32> {
    if vec.is_empty() {
        return Option::None;
    }
    let mut vec_new = vec.clone();
    vec_new.sort();

    let index = vec.len() / 2;
    println!("mid_index:{index}");

    Option::Some(vec[index])
}
