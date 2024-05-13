use std::io;
use std::cmp::Ordering;
use rand:: Rng;

fn main() {
    println!("Guessing game!");

    // Random target number
    let random_number: u32 = rand::thread_rng().gen_range(1..=100); // Generation of a random number

    //println!("The secret number is: {random_number}"); // print!() Is does not have a line break

    loop{ // loop{} is an infinite loop
        println!("Input your guess: ");

        // Input string variable
        let mut guess = String::new(); // String creation

        // Getting input from user
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line"); // Splitting the line is a good practice to avoid very long lines that result difficult to read.

        // Shadow a variable and use another datatype
        let guess: u32 = match guess.trim().parse(){// Rust lets us shadow a previous variable with the same name
            // match returns a Result type, which contains an Ok value and an Err value
            Ok(num) => num, // If the number could be parsed, return that number
            Err(_) => continue, // Else, continue
        }; //Parsing to an integer (Alternative: expect("Please type a number"); // In case the parsing fails, return this message)

        // Match comparison with less, greater or equal
        match guess.cmp(&random_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You have won!");
                break; // Interrupt the loop when the answer is correct
            }
        }
    }

}
