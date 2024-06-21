
use std::process;

// use number_guess_rs::Config;

fn main() {
    // let config = Config::build().unwrap_or_else(|err| {
    //     eprintln!("Problem parsing arguments: {err}");
    //     process::exit(1);
    // });

    if let Err(e) = number_guess_rs::run() {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}