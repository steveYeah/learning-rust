mod front_of_house;

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        // go look in the parent module
        // This is a Relative path
        super::deliver_order();
    }

    fn cook_order() {}
}

fn deliver_order() {}

// The use of pub here re-expors the module to the clients of this code
// When a client wanted to use add_to_waitlist for example they could just use
// restaurant::hosting::add_to_waitlist instead of
// restaurant::front_of_house::hosting::add_to_waitlist
// this makes the code organisation independent to how people use the code and think
// about the domain
pub use front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path: Use when we think the caller or the called will move
    // independently of each other
    //crate::front_of_house::hosting::add_to_waitlist();

    // Relative path: Use when we think the caller and the called will move
    // together, keeping the same relationship
    //front_of_house::hosting::add_to_waitlist();

    // From the use statement
    hosting::add_to_waitlist();

    // Order a breakfast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // change mind about the toast
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // will not work as seasonal_fruit is private
    //meal.seasonal_fruit = String::from("Blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
