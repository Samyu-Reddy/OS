fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
    } else if guess > secret {
    } else {
    }
}

fn main() {
    let secret_number = 42;
    let mut guess_count = 0;
    let mut guess;

    loop {
        guess = 35 + guess_count;  
        guess_count += 1;
        let result = check_guess(guess, secret_number);
        if result == 0 {
            println!("Congratulations! {} is the correct guess!", guess);
            break; 
        } else if result == 1 {
            println!("{} is too high!", guess);
        } else {
            println!("{} is too low!", guess);
        }
    }

    println!("You guessed the number in {} attempts!", guess_count);
}
