/* 
This where we create a guessing game in Rust
The user will have to guess a number between 1 and 100
The first part of the game will: 
1. ask the user to input their guess
2. check that the input is in the expected format
*/

// Import necessary libraries
use std::io; // use is similar to import in other languages

// Entry point
fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    // mutable variable to store user input
    let mut guess = String::new(); 
    /*
    let             = creates a variable
    mut             = makes the variable mutable (can be changed)
    guess           = name of the variable
    String::new()   = creates a new empty string
    */
    
    // read user input from standard input (keyboard)
    io::stdin()
        .read_line(&mut guess) // read the line and store it in guess
        .expect("Failed to read line"); // handle potential errors
    /*
    io::stdin()               = gets standard input
    .read_line()              = reads a line from standard input
    &mut guess                = passes the input to the mutable variable guess
    &                         = operator that points to where the data is stored
    .expect(...)              = handles errors if reading fails
    */

    // print the user's guess
    println!("You guessed: {}", guess);
}
