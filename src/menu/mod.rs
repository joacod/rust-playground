use crate::option::display_choices;
use colored::*;
use std::io::{self, Write};

pub fn display_welcome() {
    println!();
    println!(
        "Welcome to {} 🦀 playground!",
        "Rust".truecolor(255, 140, 0).bold()
    );
}

pub fn display_menu() {
    println!();
    println!("Choose an option:");
    display_choices();
    println!();
}

pub fn get_user_input() -> String {
    print!("Enter your choice: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    return input.trim().to_lowercase();
}
