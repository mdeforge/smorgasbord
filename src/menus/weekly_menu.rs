use crate::account::Account;
//use crate::recipe::*;
use super::menu::Menu;
use super::main_menu::MainMenu;
use inquire::Select;

#[derive(Default)]
pub struct WeeklyMenu;

impl Menu for WeeklyMenu {
    fn prompt(&self, _account: &mut Account) -> Option<Box<dyn Menu>> {
        let options = vec!["Use Smart Points", "Just Randomize", "Main Menu"];
        let ans = Select::new("How do you want to plan your weekly menu?", options).prompt().unwrap();
        match ans {
            "Use Smart Points" => {
                //
                Some(Box::new(MainMenu::default()))
            }
            "Just Randomize" => {
                //find_recipes_randomly();
                Some(Box::new(MainMenu::default()))
            }
            "Main Menu" => Some(Box::new(MainMenu::default())),
            _ => None,
        }
    }
}