/* 
This where we create a guessing game in Rust
The user will have to guess a number between 1 and 100
The first part of the game will: 
1. ask the user to input their guess
2. check that the input is in the expected format
3. generate a secret number
4. compare the user's guess to the secret number
5. Allow the user to make multiple guesses until they get it right, by using a loop
*/

// Import necessary libraries
use std::io; // use is similar to import in other languages
use rand::Rng; // for random number generation
use std::cmp::Ordering; // for comparing values

// Entry point
fn main() {
    println!("Guess the number!");

    // generate a secret number (immutable variable)
    let secret_number = rand::rng().random_range(1..=10);
    /*
    let                     = creates a variable
    secret_number           = name of the variable
    rand::rng()             = creates a random number generator
    .random_range(1..=100)  = generates a random number in the range 1 to 100 (inclusive)
    */
    //println!("The secret number is: {}", secret_number); // for debugging purposes

    let mut tries = 0;
    const MAX_TRIES: u32 = 3;

    // Create a loop to allow multiple guesses
    loop {
        // check if the user has exceeded the maximum number of tries
        if tries >= MAX_TRIES {
            println!("Sorry, you've exceeded your maximum of {} tries.", MAX_TRIES);
            println!("The secret number was: {}", secret_number);
            break;
        }

        // increment the number of tries
        tries += 1; // increment the number of tries
        println!("Attempt {} of {}", tries, MAX_TRIES);        

        // input the user's guess
        println!("Please input your guess.");

        // create an empty mutable variable
        let mut guess = String::new(); 
        /*
        let             = creates a variable
        mut             = makes the variable mutable (can be changed)
        guess           = name of the variable
        String::new()   = creates a new empty string
        */
        
        // read user input data and put it into the variable
        io::stdin()
            .read_line(&mut guess) // read the line and store the data in guess variable
            .expect("Failed to read line"); // handle potential errors
        /*
        io::stdin()               = gets standard input
        .read_line()              = reads a line from standard input
        &mut guess                = passes the input to the mutable variable guess
        &                         = operator that points to where the data is stored
        .expect(...)              = handles errors if reading fails
        */

        // convert the guess variable from a string to a number
        // create a new immutable variable that overwrites the previous mutable guess variable
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        /*
        let             = creates a variable
        guess: u32     = name of the variable with type annotation (u32 = unsigned 32-bit integer)
        guess.trim()   = removes whitespace from the input
        .parse()       = converts the string to a number
        .expect(...)   = handles errors if parsing fails
        */
        
        // print the user's guess
        println!("You guessed: {}", guess);

        // compare the guess to the secret number and break the loop if correct
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


