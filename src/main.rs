mod menu;
mod option;

fn main() {
    menu::display_welcome();

    loop {
        menu::display_menu();
        let choice = menu::get_user_input();
        let action = option::handle_select(&choice);

        match action {
            option::Action::Continue => continue,
            option::Action::Exit => break,
        }
    }
}
