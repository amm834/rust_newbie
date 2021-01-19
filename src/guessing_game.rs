use std::io;
extern crate rand;
use rand::random;
fn get_guess() -> u8 {
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
        .expect("Please enter integer.");
        println!("Your guess is: {}", guess);
        match guess.trim().parse:: < u8 > () {
            Ok(v) => return v,
            Err(e) => println!("Could not understand -> {}", e)
        }
    }
}

fn handle_guess(guess: u8, correct: u8)->bool {
    if guess > correct {
        println!("Guessing number is too hight.");
        false
    }else if guess < correct {
        println!("Guessing number is too low.");
        false
    }else {
        println!("You got it!");
        true
    }
}

fn run() {
    println!("Welcome from gusessimg game.!");
    let correct = random:: < u8 > ();
    // println!("Correct answer is: {}", correct);
    loop {
        let guess = get_guess();
        if handle_guess(guess, correct) {
            break;
        }
    }
}