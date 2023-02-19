//This line brings the Rng trait into scope. The Rng trait defines methods for generating random numbers.
use rand::Rng;
//This line brings the Ordering enum from the standard library's cmp module into scope. The Ordering enum is used to compare values and determine their order relative to each other.
use std::cmp::Ordering;

// This line brings the io module from the standard library into scope. 
// he io module provides input and output functions for reading from the standard input (i.e., the keyboard) 
// and writing to the standard output (i.e., the terminal).
use std::io;

// line starts the definition of the main function, which is the entry point of every Rust program.
fn main() {

    //This line uses the println! macro to print the message "Guess the number!" to the standard output.
    println!("Guess the number!");

    //This line generates a random number between 1 and 100 and stores it in a variable named secret_number.
    //The rand::thread_rng function returns a random number generator that is local to the current thread.
    //The gen_range method is then called on the returned random number generator to generate the random number.
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // This is the start of a loop that continues until the user correctly guesses the number.
    loop {

        //This line uses the println! macro to print the message "Please input your guess." to the standard output.
        println!("Please input your guess.");

        // This line declares a mutable variable named guess with the type String. 
        // The String::new method is called to create a new, empty string.
        let mut guess = String::new();

        //This line reads a line of text from the standard input and stores it in the guess variable. 
        //The read_line method is called on the result of io::stdin, which returns a handle to the standard input.
        //The &mut guess argument is a mutable reference to the guess variable, which allows the read_line method to modify the contents of the variable.
        //The expect method is called on the result of read_line to handle any errors that might occur. If an error occurs, the program will terminate and print the message "Failed to read line".
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        // This line attempts to parse the contents of the guess variable as an integer and store the result in a new variable of type u32. 
        // The trim method is called on the guess variable to remove any leading or trailing whitespace, and the parse method is then called to parse the contents of the guess variable as an integer. 
        // The result of parse is a Result
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        //This code is part of a loop that compares the user's guess to a secret number. 
        //If the guess is less than the secret number, it prints "Too small!" 
        //If the guess is greater than the secret number, it prints "Too big!". 
        //If the guess is equal to the secret number, it prints "You win!" and breaks out of the loop.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}
