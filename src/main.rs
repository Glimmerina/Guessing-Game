use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number of pixies!");

    let pixies = rand::thread_rng().gen_range(1..=100);//immutable variable

    loop{ 
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
            Ordering::Less => println!("Not enough pixies!"),
            Ordering::Greater => println!("Too many pixies!"),
            Ordering::Equal => {
                println!("You win! Keep on sparkling, darling!");
                break;
            }
        }
    }
}
