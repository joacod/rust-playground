use colored::*;

pub const CHOICES: [Choice; 3] = [Choice::Green, Choice::Blue, Choice::Quit];

pub enum Choice {
    Green,
    Blue,
    Quit,
}

pub enum Action {
    Continue,
    Exit,
}

trait ChoiceHandler {
    fn display(&self);
    fn select(&self) -> Action;
}

impl ChoiceHandler for Choice {
    fn display(&self) {
        match self {
            Choice::Green => println!("1. {} Option", "Green".green()),
            Choice::Blue => println!("2. {} Option", "Blue".blue()),
            Choice::Quit => println!("q. Quit"),
        }
    }

    fn select(&self) -> Action {
        match self {
            Choice::Green => {
                println!("{}", "You choose 1 🟢".green());
                Action::Exit
            }
            Choice::Blue => {
                println!("{}", "You choose 2 🔵".blue());
                Action::Exit
            }
            Choice::Quit => {
                println!("Thanks for using our program! 👋");
                Action::Exit
            }
        }
    }
}

pub fn display_choices() {
    for choice in &CHOICES {
        choice.display();
    }
}

pub fn handle_select(choice: &str) -> Action {
    let option = match choice {
        "1" => Choice::Green,
        "2" => Choice::Blue,
        "q" => Choice::Quit,
        _ => {
            println!("{}", "❌ Invalid choice!".red());
            return Action::Continue;
        }
    };

    return option.select();
}
