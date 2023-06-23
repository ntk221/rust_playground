mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

use std::collections::HashMap;

fn main() {
    eat_at_restaurant();
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// 一方で、構造体やenumその他の要素をuseで持ち込むときは、フルパスを書くのが慣例的です。
