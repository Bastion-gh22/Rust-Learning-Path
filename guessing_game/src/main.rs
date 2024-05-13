use std::io;

fn main() {
    println!("Guessing game!");
    println!("Input your guess: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line"); // Splitting the line is a good practice to avoid very long lines that result difficult to read.

    println!("Guessed word: {guess}");

}
