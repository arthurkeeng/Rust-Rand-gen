

// this line brings the io from the standard library into scope
use std::io;
// use rand;
use rand::Rng;
use std::cmp::Ordering;
// extern crate rand;
// this is rust's default entry function

fn main(){
    // this helps to generate a random number in the current thread
    let secret_number = rand::thread_rng().gen_range(1, 101);
    // this puts the game in a loop in situations where the user inputs or guesses are wrong
    loop{
    // displays to the user to input the guess
   println!("please enter your guess");
    
//    creates a mutable variable for an empty string
   let mut guess = String::new();

//    takes the user input and passes it to the mutable guess reference
    io::stdin().read_line(&mut guess).expect("failed to read input");

    // this trims the guess variable of whitespaces and newline gotten from the read_line function
    let guess : u32 = match guess.trim().parse(){
        // so this returns an enum of two values , ok and err. We do a match to see which one was returned
        Ok(num) => num, 
        Err(_) => {
            println!("the input you entered is not a number");
            continue;
    },
    };
    // we do a match here between the guess and the secret number
    match guess.cmp(&secret_number) {
        // Ordering is on the cmp function and returns an enum with 3 results
        // we do a match to see which one was returned;
        Ordering::Less => println!("the number you entered was less"),
        Ordering::Greater => println!("the number you entered is greater"),
        Ordering::Equal => {
            println!("you guessed the secret number correctly , you win");
            break;
        }
    }
    }

 
}


