mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

//pub can be used to make a function public, but it doesn't make the parent module public.
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

mod customer {
    pub fn eat_at_restaurant() {
        //error: cannot find function `hosting` in this scope
        // hosting::add_to_waitlist(); 
    }
}
}

// use std::fmt::Result;
// in this case, the `Result` from `std::fmt` is shadowed by the `Result` from `std::io`
// use std::io::Result as IoResult;

//or
use std::fmt;
// use std::io;

fn function1() -> fmt::Result {
    // --생략--
    return Ok(());
}

fn function2() -> io::Result<()> {
    // --생략--
    return Ok(());
}

// use std::cmp::Ordering;
// use std::io;
// use std::io::Write;

// use nested paths to clean up multiple use statements that share a common prefix
use std::{cmp::Ordering, io};
// use std::io::{self, Write};

// glob(*) operator to bring all public items defined in a path into scope
use std::collections::*;

mod front_of_house2;

pub use crate::front_of_house2::hosting2;

pub fn eat_at_restaurant2() {
    hosting2::add_to_waitlist();
}