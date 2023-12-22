use std::ops::Add;

fn main() {
    println!("Hello, world!");
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    println!("====enum=====");

    println!("Penny {}", value_in_cents(Coin::Penny));
    println!("Penny {}", value_in_cents(Coin::Nickel));
    println!("Penny {}", value_in_cents(Coin::Quarter(UsState::Alabama)));

    println!("====option=====");
    println!("Option {:?}", plus_one(Option::Some(1)));
    println!("Option {:?}", plus_one(Some(1)));
    println!("Option {:?}", plus_one(None));
    println!("Option {:?}", plus_one(Option::None));

    println!("通配模式和 _ 占位符");
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    println!("if let 简洁控制流");
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    let mut config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
    config_max = None;
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    } else {
        println!("IS NONE");
    }
    println!("if let 简洁控制流 end");
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}

fn plus_one(a: Option<i32>) -> Option<i32> {
    match a {
        None => None,
        Some(x) => Some(x + 1),
    }
}

/* 州
*/
#[derive(Debug)] // 这样可以立刻看到州的名称
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(value: Coin) -> i32 {
    match value {
        Coin::Penny => {
            print!("==Penny ");
            1
        }
        Coin::Nickel => 2,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            print!("Quarter state {state:?} ");
            15
        }
    }
}
