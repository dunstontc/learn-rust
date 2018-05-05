extern crate rand;

use std::io;
// Ordering is another enum, like Result, but the variants for Ordering are Less, Greater, and Equal.
// These are the three outcomes that are possible when you compare two values.
use std::cmp::Ordering;
// Rng is a *trait* that defines methods that random number generators implement, and this trait must be in scope for us to use those methods.
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // In Rust, variables are immutable by default.
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // The loop keyword gives us an infinite loop.
    loop {
        println!("Please input your guess.");

        // String is a string type provided by the standard library that is a growable, UTF-8 encoded bit of text.
        // The `::` syntax indicates that `new` is an *associated function* (static method).
        let mut guess = String::new();

        // The `&` indicates that this argument is a *reference*,
        // which gives you a way to let multiple parts of your code access one
        // piece of data without needing to copy that data into memory multiple times.
        // Like variables, references are immutable by default.
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

            // read_line returns an `io::Result` value. (an enum)
            // An enumeration is a type that can have a fixed set of values,
            // and those values are called the enum’s *variants*.
            // For Result, the variants are `Ok` or `Err`.

        // The parse method on strings parses a string into some kind of number.
        // Because this method can parse a variety of number types,
        // we need to tell Rust the exact number type we want by using let guess: u32.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,      // `Ok` indicates the operation was successful, and inside the `Ok` variant is the successfully generated value.
            Err(_) => continue,  // `Err` means the operation failed, and `Err` contains information about how or why the operation failed.
        };

        println!("You guessed: {}", guess);

        // The cmp method compares two values and can be called on anything that can be compared.
        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }
    }
}
