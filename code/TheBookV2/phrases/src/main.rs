extern crate phrases;

use phrases::english::{greetings, farewells};
use phrases::japanese;

fn main() {
    println!("Hello in English: {}", greetings::hello());
    println!("Goodbye in English: {}", farewells::goodbye());

    println!("Hello in Japanese: {}", japanese::greetings::hello());
    println!("Goodbye in Japanese: {}", japanese::farewells::goodbye());
}
