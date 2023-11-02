use std::env;
use std::process;

use minigrep::run;
use minigrep::Config;

fn main() {
    // env::set_var("IGNORE_CASE", "0");
    // env::remove_var("IGNORE_CASE");

    let args: Vec<String> = env::args().collect();

    // let config = Config::build(&args).unwrap_or_else(|err| {
    //     eprintln!("Problem parsing arguments: {}", err);
    //     process::exit(1);
    // });
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for: {}", config.query);
    println!("In file: {}", config.file_path);

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
