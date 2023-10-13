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
}

fn another_function(x: i32) -> i32 {
    println!("Another function: {x}.");

    45
}