use std::{fs::File, io::ErrorKind, io::{self, Read}, error::Error};

fn main() {
    // panic!("crash and burn");

    // let v = vec![1, 2, 3];
    // v[99];

    // let greeting_file_result = File::open("hello.txt");
    // let greeting_file_result = File::open("hello.txt").unwrap(); // panic!
    // let greeting_file_result = File::open("hello.txt").expect("Failed to open hello.txt"); // panic!

    // panic if error
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // };

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error)
    //         }
    //     }
    // };

    // using closures
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         }),
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error)
    //         }
    //     },
    // };

    // let x = read_username_from_file();

    // fs::read_to_string("hello.txt");

    let y = read_username_from_file2();
    println!("Value of y: {:?}", y);

    let x = last_char_of_first_line("");
    println!("Value of x: {:?}", x);

    let z = read_file();
    println!("Value of z: {:?}", z);
}


fn read_username_from_file () -> Result<String, io::Error> {
    let user_file_result = File::open("hello.txt");

    let mut username_file = match user_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2 () -> Result<String, io::Error> {
    // let mut username_file = File::open("hello.txt")?;
    // let mut username = String::new();
    // username_file.read_to_string(&mut username)?;
    // Ok(username)
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

// any kind of error
fn read_file() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
}