fn main() {
    println!("Hello, world!");

    // let x = 3;

    let x = 23; // shadowing

    {
        let x = 123;
        println!("the value is {x}");
    }

    println!("the value is {x}");

    // perform few transformations, then print the old value
    let spaces = "   ";
    let spaces = spaces.len();

    println!("the value is {spaces}");

    // let mut testing_panicking: u8 = 255;
    // testing_panicking += 10;
    // println!("the value is {testing_panicking}");

    let test = 12.3 / 4.9;
    println!("the value is {test}");

    let x = another_function(123);
    println!("the value is {x}");

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("the value is {number}");

    // working with counter
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 2 {
            break counter * 2;
        }
    };

    println!("the value is {result}");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is {}", a[index]);

        index += 1;
    }

    println!("the value is {} ", a[0]);

    let mut s = String::from("hello world");
    s.push_str(", how are you?");
    println!("the value is: {s}");

    // preventing the double free error
    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("the value is: {s1}"); // value borrowed here after move

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("the value is: {s1} and {s2}"); // deep copy

    // let s3 = String::from("hello");
    // takes_ownership(s3);
    // println!("the value is: {s3}"); // value borrowed here after move

    let s4 = String::from("hello");
    let (s5, len) = calculate_length(s4);
    println!("the value is: {s5} and {len}");

    let s6 = String::from("hello");
    // The &s6 syntax lets us create a reference that refers to the value of s1 but does not own it.
    let len = calculate_length_ref(&s6);
    println!("the value is: {s6} and {len}");

    // if you have a mutable reference to a value, you can have no other references to that value.
    // prevent data races at compile time
    // let mut s7 = String::from("hello");
    // let r1 = &mut s7;
    // let r2 = &mut s7;

    // println!("{}, {}", r1, r2);

    let mut s7 = String::from("hello");
    {
        let r1 = &mut s7;
    } // r1 is out of scope here
    let r2 = &mut s7;

    // println!("{}, {}", r1, r2);

    let mut s8 = String::from("hello");

    change(&mut s8);
    println!("the value is: {s8}");

    let s9 = String::from("hello world");
    let slice = &s9[0..2];
    println!("the value is: {slice}");
    let slice = &s9[..2];
    println!("the value is: {slice}");

    let mut s10 = String::from("hello world");
    let word = first_word(&s10);
    // cannot borrow `s10` as mutable because it is also borrowed as immutable mutable borrow occurs here
    // s10.clear();
    println!("the value is: {word}");
}

fn another_function(x: i32) -> i32 {
    println!("Another function: {x}.");

    45
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn calculate_length_ref(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
