mod menu;
use std::io::{self, Write};

fn main() {
    menu::display_welcome();

    loop {
        menu::display_menu();
        let choice = get_user_input();
        let exit = menu::handle_option(&choice);

        if exit {
            break;
        }
    }
}

fn get_user_input() -> String {
    print!("Enter your choice: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    return input.trim().to_lowercase();
}
