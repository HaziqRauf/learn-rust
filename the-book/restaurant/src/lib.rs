mod front_of_house;

pub use crate::front_of_house::hosting;

mod back_of_house {
    pub struct Breakfast { //often useful without their fields being public
        pub toast: String,
        seasonal_fruit: String,
    }
    pub enum Appetizer { //aren't very useful unless their variants are public
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer (toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    //absolute path
    //crate::front_of_house::hosting::add_to_waitlist();

    //relative path
    // front_of_house::hosting::add_to_waitlist();

    //order a breakfast in the summer with Rye toast
    // let mut meal = back_of_house::Breakfast::summer("Rye");
    //change our mind about what bread we'd like
    // meal.toast = String::from("Wheat");
    // println!("I'd like {} toast please", meal.toast);

    // let order1 = back_of_house::Appetizer::Soup;
    // let order2 = back_of_house::Appetizer::Salad;
}
