use rand::Rng;
use std::io;
use std::cmp::Ordering;

pub fn generate_random_number(from: u32, under: u32) -> u32 {
    return rand::thread_rng().gen_range(from..under);
}

pub fn prompt_user_for_guess() -> u32 {
    println!("Please input your guess.");
    let mut guess = String::new();
    loop {
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        match guess.trim().parse() {
            Ok(num) => {
                println!("You guessed: {}", num);
                return num;
            },
            Err(_) => continue,
        };
    }
}

pub fn user_won_game(guess: &u32, secret_number: &u32) -> bool {
    let mut won_game = false;

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            won_game = true;
        }
    }

    return won_game;
}
