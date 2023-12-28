use colored::*;

pub fn display_welcome() {
    println!(
        "Welcome to {} 🦀 playground!",
        "Rust".truecolor(255, 140, 0).bold()
    );
}

pub fn display_menu() {
    println!();
    println!("Choose an option:");
    println!("1. {} Option", "Green".green());
    println!("2. {} Option", "Blue".blue());
    println!("q. Quit");
    println!();
}

// true if exit, false if continue
pub fn handle_option(choice: &str) -> bool {
    match choice {
        "1" => {
            println!("{}", "You choose 1 🟢".green());
            true
        }
        "2" => {
            println!("{}", "You choose 2 🔵".blue());
            true
        }
        "q" => {
            println!("Thanks for using our program! 👋");
            true
        }
        _ => {
            println!("{}", "❌ Invalid choice!".red());
            false
        }
    }
}
