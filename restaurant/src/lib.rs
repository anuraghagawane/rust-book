use std::fmt::Result;
use std::io::Result as IoResult;
// use std::{cmp::Ordering, io};
use std::io::{self, Write};
use std::collections::*;

fn deliver_order() {}

mod front_of_house; 

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
    fn cook_order() {}
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
    // use statement to create a shortcut for hosting module
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Ryp");
    meal.toast = String::from("wheat");

    println!("I'd like {} toast please", meal.toast);
    // meal.seasonal_fruit = String::from("blueberries");
    
    // enums variant becomes public if enum is declared public
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
