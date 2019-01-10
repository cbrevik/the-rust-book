mod sound;

mod plant {
    pub struct Vegetable {
        pub name: String,
        id: i32,
    }

    impl Vegetable {
        pub fn new(name: &str) -> Vegetable {
            Vegetable {
                name: String::from(name),
                id: 1,
            }
        }
    }
}

mod menu {
    // All variants of enums are automatically public
    // if enum is public
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

use crate::sound::instrument;
//use self::sound::instrument; // relative path
use crate::sound::instrument::clarinet;

fn main() {
    crate::sound::instrument::clarinet();

    sound::instrument::clarinet();

    let mut v = plant::Vegetable::new("squash");

    v.name = String::from("butternut squash");
    println!("{} are delicious", v.name);

    // The next line won't compile if we uncomment it:
    // println!("The ID is {}", v.id);

    let order1 = menu::Appetizer::Soup;
    let order2 = menu::Appetizer::Salad;

    // because of:
    // use crate::sound::instrument;
    instrument::clarinet();

    // because of:
    // use crate::sound::instrument::clarinet;
    clarinet();

    // although:
    // For functions, it’s considered idiomatic to specify the function’s parent module with use, and then specify the parent module when calling the function. Doing so rather than specifying the path to the function with use, as Listing 7-16 does, makes it clear that the function isn’t locally defined, while still minimizing repetition of the full path.

    // For structs, enums, and other items, specifying the full path to the item with use is idiomatic. For example, Listing 7-17 shows the idiomatic way to bring the standard library’s HashMap struct into scope.

    use std::collections::HashMap;

    let mut map = HashMap::new();
    map.insert(1, 2);

    performance_group::clarinet_trio();
    performance_group::instrument::clarinet();

    // Pull out several from same module in one line
    use std::{cmp::Ordering, io};

    // Pulls out both io it-self and sub-path Write
    use std::io::{self, Write};

    // Glob: "all"
    use std::collections::*;
}

mod performance_group {
    // re-exporting:
    pub use crate::sound::instrument;

    pub fn clarinet_trio() {
        instrument::clarinet();
        instrument::clarinet();
        instrument::clarinet();
    }
}
