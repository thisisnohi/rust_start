use lib2::hello as hello2;
use lib3::front_of_house;
use liba::hello;

fn main() {
    println!("Hello, world!");

    hello();
    hello2();

    front_of_house::hosting::add_to_waist();
}
