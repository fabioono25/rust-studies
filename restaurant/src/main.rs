use std::collections::HashMap;

use std::fmt::Result;
use std::io::Result as IoResult;

use rand::Rng;

// use std::cmp::Ordering;
// use std::io;
// use std::{cmp::Ordering, io};
use std::io::{self, Write};

use std::collections::*;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);

    let secret_number = rand::thread_rng().gen_range(1..=101);
    println!("the value is {}", secret_number);

    // consume the eat_at_restaurant function, present in the lib.rs file
    restaurant::eat_at_restaurant();

}

// fn function1() -> fmt::Result {
//   Ok(())
// }

// fn function2() -> io::Result<()> {
//   Ok(())
// }

fn function1() -> Result {
    Ok(())
}

fn function2() -> IoResult<()> {
    Ok(())
}
