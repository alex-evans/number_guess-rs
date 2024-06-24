
// use clap::Parser;
use std::error::Error;
use std::io;
use rand::Rng;

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
    println!("Hello and welcome to Number Guessing Game!");
    
    let player_name = get_player_name().unwrap();
    
    println!("Welcome, {}!", player_name);

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let max_guesses = 10;

    run_guess_loop(secret_number, max_guesses).unwrap();
    

    Ok(())
}

fn get_player_name() -> Result<String, Box<dyn Error>>{
    println!("To start, please enter your name: ");
    let mut player_name = String::new();

    io::stdin()
        .read_line(&mut player_name)
        .expect("Failed to read line");

    let player_name: String = match player_name.trim().parse() {
        Ok(player_name) => player_name,
        Err(_) => {
            println!("Please enter a valid name");
            return get_player_name();
        }
    };
    
    Ok(player_name)
}


fn run_guess_loop(secret_number: u32, max_guesses: u32) -> Result<(), Box<dyn Error>> {
    let mut guess = String::new();
    let mut guesses = 0;

    loop {
        if guesses >= max_guesses {
            println!("Game Over! You have reached the maximum number of guesses.");
            println!("The secret number was: {}", secret_number);
            break;
        }

        println!("Please enter your guess: ");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(guess) => guess,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                println!("You guessed the number in {} tries", guesses);
                break;
            }
        }

        guesses += 1;
    }

    Ok(())
}