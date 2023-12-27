pub enum Appetizer {
    Soup,
    Salad,
}

pub fn add_to_waist() {
    super::serving::take_order();
    crate::front_of_house::serving::take_order();
}

fn seat_at_table() {}
