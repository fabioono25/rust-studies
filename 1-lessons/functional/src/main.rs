use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    /* using closures */
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        num * 2
    };
    println!("expensive_closure(10) = {}", expensive_closure(10));

    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    let n = example_closure(5.to_string());
    println!("s = {}, n = {}", s, n);

    let mut list = vec![1,2,3];
    println!("before defining closure: {:?}", list);

    let mut bollows_mutability = || list.push(4);
    bollows_mutability();
    println!("after calling closure: {:?}", list);

    let only_borrows = || println!("list: {:?}", list);
    println!("Before calling closure: {:?}", list);

    only_borrows();
    println!("After calling closure: {:?}", list);

    thread::spawn(move || {
        println!("Hello from a thread {:?}!", list);
    }).join().unwrap();
}
