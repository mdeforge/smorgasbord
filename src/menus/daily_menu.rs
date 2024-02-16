use crate::account::Account;
//use crate::recipes::*;
use super::menu::Menu;
use super::main_menu::MainMenu;
use inquire::Select;

#[derive(Default)]
pub struct DailyMenu;

impl Menu for DailyMenu {
    fn prompt(&self, _account: &mut Account) -> Option<Box<dyn Menu>> {
        let options = vec!["Use Smart Points", "Just Randomize", "Main Menu"];
        let ans = Select::new("Choose", options).prompt().unwrap();
        match ans {
            "Use Smart Points" => {
                //find_recipes_using_smart_points();
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