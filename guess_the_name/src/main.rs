use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guessing starts here !!!");

    let secret_number = rand::thread_rng().gen_range(1..=10);

    println!("The secret number is: {secret_number}");

    // in Rust variables are immutable by default so guess is declared as mut ,
    // also binding result of String::new to variable guess
    loop {
        println!("Enter a number to compare");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess_int: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You Guessed {guess}");

        match guess_int.cmp(&secret_number) {
            Ordering::Equal => 
            {
                println!("you Win");
                break;
            }
            Ordering::Greater => println!("Too big"),
            Ordering::Less => println!("Too Small"),
        }
    }
}
