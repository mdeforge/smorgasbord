use crate::account::Account;
use super::menu::Menu;
use super::setup_menu::SetupMenu;
use inquire::Text;

#[derive(Default)]
pub struct NewAccountMenu;

impl Menu for NewAccountMenu {
    // TODO(mdeforge): How to cancel?
    fn prompt(&self, account: &mut Account) -> Option<Box<dyn Menu>> {
        let name = Text::new("Please enter a name:").prompt().unwrap();

        // Check if empty
        if name.is_empty() {
            println!("Name cannot be empty, please enter a valid name.");
            return Some(Box::new(NewAccountMenu::default()));
        }

        match account.save() {
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