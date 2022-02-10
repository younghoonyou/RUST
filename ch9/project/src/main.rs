extern crate restaurant;//크레이트 접근
use restaurant::{front_of_house::serving,back_of_house};
fn main() {
    restaurant::eat_at_restaurant();
    back_of_house::fix_incorrect_order();
}
