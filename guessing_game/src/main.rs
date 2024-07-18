fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    let secret = 42; // Hard-coded secret number
    let mut guess_count = 0;
    let mut guess = 0; // Initial guess value, you can change it to simulate user input

    loop {
        guess_count += 1;

        let result = check_guess(guess, secret);

        if result == 0 {
            println!("Correct! The secret number is {}.", secret);
            break;
        } else if result == 1 {
            println!("Too high!");
        } else {
            println!("Too low!");
        }

        // Change the guess value to simulate different user inputs
        guess += 1; // Incrementing guess to simulate user input
    }

    println!("It took {} guesses to find the secret number.", guess_count);
}