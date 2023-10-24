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

    let mut user = User {
        email: String::from("abc"),
        username: String::from("xyz"),
        active: true,
        sign_in_count: 1,
    };

    user.email = String::from("another email");

    println!("the value is: {}", user.email);

    let user2 = build_user(String::from("abc"), String::from("xyz"));
    println!("the value is: {}", user2.email);

    let user3 = User {
        email: String::from("abcd"),
        username: String::from("xyz"),
        ..user2
    };
    println!("the value is: {}", user3.email);

    let black = Color(0, 0, 0);

    let rect = (30, 50);
    println!("the value is: {}", area(rect));

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("the value is: {}", area2(&rect2));
    println!("rect2 is {:?}", rect2);

    let rect3 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("Area of rect3 is {}", rect3.area());
    println!("Width of rect3 is {}", rect3.width());

    let rect4 = Rectangle::square(30);
    println!("Area of rect4 is {}", rect4.area());

    // working with enums
    let four = IpAddrKind::V4;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr2::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some('e');
    let absent_number: Option<i32> = None;
    let coin = Coin::Penny;
    let value = value_in_cents(coin);
    let value2 = value_in_cents(Coin::Quarter(UsState::Alabama));

    let five = Some(5);
    let six = plus_one(five);

    let config_max = Some(3u8);

    if let Some(max) = config_max {
        println!("Max is {}", max);
    } else {
        println!("No max provided");
    }

    // working with traits
    let article = NewsArticle {
        headline: String::from("headline"),
        location: String::from("location"),
        author: String::from("author"),
        content: String::from("content"),
    };

    println!("New article available! {}", article.summarize());
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

fn build_user(email: String, username: String) -> User {
    User {
        email, // field init shorthand
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// tuple struct
struct Color(i32, i32, i32);

// unit-like struct
struct UnitLikeStruct;

fn area(dimension: (u32, u32)) -> u32 {
    dimension.0 * dimension.1
}

fn area2(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        // method
        self.width * self.height
    }

    fn width(&self) -> u32 {
        // method
        self.width
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,                       // no data associated
    Move { x: i32, y: i32 },    // anonymous struct
    Write(String),              // single string
    ChangeColor(i32, i32, i32), // three i32 values
}

impl Message {
    fn call(&self) {
        println!("call inside message");
    }
}

// enum Option<T> {
//     Some(T),
//     None,
// }

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        // match is exhaustive
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            // match is exhaustive
            println!("State quarter from {:?}!", state);
            25
        }
        _ => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        // match is exhaustive
        None => None,
        Some(i) => Some(i + 1),
    }
}

// working with traits
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!(
            "(Read more from {}...)",
            self.summarize_author()
        )
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!(
            "{}, by {} ({})",
            self.headline,
            self.author,
            self.location
        )
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}