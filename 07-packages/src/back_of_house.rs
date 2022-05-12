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
    // with super we can reference the level of the parent namespace relative to this one
    // it's like doing a .. but for paths
    // the parent lives is the module 'crate', so the root
    // this is why we can access the server_order function because it is also in the
    // module crate
    super::serve_order();
}

fn cook_order() {}
