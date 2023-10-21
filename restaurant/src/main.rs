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

    // working with collections
    let mut v: Vec<i32> = Vec::new();
    v.push(4);
    let v2 = vec![1, 2, 3];

    println!("v is {:?}", v[0]);
    println!("v2 is {:?}", v2);

    let does_not_exist = v.get(100); // None
                                     // let does_not_exist2 = &v[100]; // panicking

    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6); // cannot borrow `v` as mutable because it is also borrowed as immutable mutable borrow occurs here
    // println!("The first element is: {first}");

    let mut v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 50; // dereference operator
        println!("{}", i);
    }

    // using enums to store different types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(3.14),
        SpreadsheetCell::Text(String::from("blue")),
    ];
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
