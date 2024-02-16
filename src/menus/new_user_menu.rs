use crate::account::Account;
use super::menu::Menu;
use super::setup_menu::SetupMenu;
use inquire::Text;
use std::path::Path;

#[derive(Default)]
pub struct NewUserMenu;

impl Menu for NewUserMenu {
    // TODO(mdeforge): How to cancel?
    fn prompt(&self, user: &mut Account) -> Option<Box<dyn Menu>> {
        let name = Text::new("Please enter a name:").prompt().unwrap();

        // Check if empty
        if name.is_empty() {
            println!("Name cannot be empty, please enter a valid name.");
            return Some(Box::new(NewUserMenu::default()));
        }

        user.name = name.clone();
        let path = format!("./data/{}.user", name.clone());
        match user.save(Path::new(&path)) {
            Ok(()) => {
                println!("Account has been created.");
                Some(Box::new(SetupMenu::default()))
            },
            _ => {
                println!("Failed to create account.");
                None
            }
        }

    }
}