use std::env;
use std::process;

use minigrep::Config;
use minigrep::run;

fn main() {
    let args: Vec<String> = env::args().collect();

    // // let config = Config::new(&args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // println!("{:?}", args);
    println!("Searching for: {}", config.query);
    println!("In file: {}", config.file_path);

    // let contents =
    //     fs::read_to_string(config.file_path).expect("Should habe been able to read the file");

    // println!("With text:\n{}", contents);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}