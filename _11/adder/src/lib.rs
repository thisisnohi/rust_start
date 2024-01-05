use std::collections::hash_map::Values;
use std::panic::panic_any;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(left: usize) -> usize {
    left + 2
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value 必须在1-100之间!")
        }
        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn exploration() {
        println!("这里是正常执行的输出");
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn another() {
        println!("这里是错误执行的输出");
        //panic!("Make this test fail..")
    }

    #[test]
    fn test_can_hold() {
        let rect1 = Rectangle {
            width: 8,
            height: 7,
        };
        let rect2 = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(rect1.can_hold(&rect2), "rect1 can't hold rect2")
    }

    // #[test]
    // #[should_panic(expected = "less than or equal to 100")]
    // fn test_guess() {
    //     Guess::new(200);
    // }

    #[test]
    #[ignore]
    fn it_works_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
