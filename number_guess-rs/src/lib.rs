
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
    let mut top_score = 0;
    println!("Welcome, {}!", player_name);

    let mut played_already = false;
    loop {
        if played_already {
            println!("Would you like to play again? (y/n)");
            let mut play_again = String::new();
            io::stdin()
                .read_line(&mut play_again)
                .expect("Failed to read line");
            
            match play_again.trim() {
                "n" => {
                    println!("Goodbye, {}!", player_name);
                    break;
                },
                "y" => {
                    println!("Great! Let's play again!");
                    let top_score = run_guess_loop(top_score).unwrap();
                    continue;
                },
                _ => {
                    println!("Please enter a valid option");
                    continue;
                }
            }
        } else {
            let top_score = run_guess_loop(top_score).unwrap();
            played_already = true;
        }
    }

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


fn run_guess_loop(top_score: u32) -> Result<u32, Box<dyn Error>> {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let max_guesses = 10;
    let mut guesses = 0;

    loop {
        let mut guess = String::new();

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
                println!("Your guess was: {}", guess.trim());
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
                if top_score == 0 {
                    top_score = guesses;
                } else if guesses < top_score {
                    top_score = guesses;
                    println!("Congratulations! You have set the top score!");
                } else {
                    println!("The top score is: {}", top_score);
                    println!("Keep trying");
                }
                break;
            }
        }

        guesses += 1;
    }

    Ok(top_score)
}