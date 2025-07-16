use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number of pixies!");

    let pixies = rand::thread_rng().gen_range(1..=100);//immutable variable

    let mut guesses  = 5;

    loop{ 
        if guesses > 0 {
            //If the user has lives left, prompt them to input a guess
            println!("Please input your guess.");
            let mut guess= String::new(); //mutable variable

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

                let guess: u32 = match guess.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

            println!("You guessed: {guess}");

            match guess.cmp(&pixies){
                Ordering::Less => {
                    println!("Not enough pixies!");
                    guesses = guesses -1;
                    println!("You have {} guesses left.", guesses);
                }
                Ordering::Greater => {
                    println!("Too many pixies!");
                    guesses = guesses -1;
                    println!("You have {} guesses left.", guesses);
                }
                Ordering::Equal => {
                    println!("You win! Keep on sparkling, darling!");
                    break;
                }
            }
    }
        else {
            //If the user has no lives left, end the game.
            println!("You have run out of guesses! The redcaps have come for you!");
            break;
    }
    }
}
