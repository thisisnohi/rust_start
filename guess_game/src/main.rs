// use rand::Rng;
// use std::cmp::Ordering;
// use std::io::stdin;

fn main() {
    // println!("Guess the number..");
    // println!("Input you choice..");
    //
    // let secret_num = rand::thread_rng().gen_range(1..101);
    // println!("secret_num {secret_num}");
    //
    // let mut guess = String::new();
    // stdin().read_line(&mut guess).expect("Fail to read guess");
    //
    // println!("You guessed {guess}");
    //
    // // 版本一
    // // let guess_number = i32::from_str_radix(&guess.trim(), 10).expect("Failed to convert a number");
    // // if guess_number == secret_num {
    // //     println!("You win");
    // // } else if guess_number > secret_num {
    // //     println!("Too big");
    // // } else {
    // //     println!("You small");
    // // }
    //
    // // 版本二
    // let guess: i32 = guess.trim().parse().expect("Failed to convert a number");
    // match guess.cmp(&secret_num) {
    //     Ordering::Less => println!("too small"),
    //     Ordering::Greater => println!("too Big"),
    //     Ordering::Equal => println!("You win"),
    // }

    let mut s1 = String::from("hello");
    let len = calculate_length(&mut s1);
    println!("{s1} len {len}");

    let str = String::from("aaa bbb ccc");
    let str2 = String::from("你好 中国");

    let aaa = &str[0..3];
    let bbb = &str[4..7];
    println!("aaa: {aaa}");
    println!("bbb: {bbb}");

    println!("[..=4]: {}", &str[..=4]);
    println!("[..3]: {}", &str[..3]);
    println!("[4..]: {}", &str[4..]);
}

fn calculate_length(s: &mut String) -> usize {
    s.push_str(", world");
    s.len()
}

fn fibonacci(int: i32) -> i32 {
    if int > 1 {
        return int + fibonacci(int - 1);
    } else {
        1
    }
}

fn print_labeled_measurement(value: i32, label: char) {
    println!("Tmeasurement is : {value}{label}")
}
