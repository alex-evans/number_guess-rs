
use clap::Parser;
use std::error::Error;
use std::io;

// #[derive(Parser, Debug)]
// #[command(version, about, long_about = None)]
// pub struct Config {
//     player_name: String,
// }

// impl Config {
//     pub fn build() -> Result<Config, &'static str> {
//         let args = Config::parse();
//         Ok(args)
//     }
// }

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("Hello and welcom to Number Guessing Game!");
    println!("To start, please enter your name: ");
    let mut player_name = String::new();

    io::stdin()
        .read_line(&mut player_name)
        .expect("Failed to read line");

    let player_name: String = match player_name.trim().parse() {
        Ok(player_name) => player_name,
        Err(_) => {
            println!("Please enter a valid name");
            return Ok(());
        }
    };

    Ok(())
}