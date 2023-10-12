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
}
