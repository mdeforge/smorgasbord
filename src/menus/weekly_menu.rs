use crate::account::Account;
//use crate::recipe::*;
use super::menu::Menu;
use super::main_menu::MainMenu;
use inquire::Select;

#[derive(Default)]
pub struct WeeklyMenu;

fn generate_daily_menu_using_smart_points() {
    println!("Daily smart points!");
}

fn generate_weekly_menu_using_smart_points(account: &mut Account) {
    for user in account.get_users() {
        let data = account.get_user(user);
        //println!("{}: {}", user, data.unwrap().daily_points);
    }
}

fn generate_daily_menu_randomly() {
    println!("Daily randomly!")
}

fn generate_weekly_menu_randomly() {
    println!("Weekly randomly!")
}

impl Menu for WeeklyMenu {
    fn prompt(&self, account: &mut Account) -> Option<Box<dyn Menu>> {
        let options = vec!["Use Smart Points", "Just Randomize", "Main Menu"];
        let ans = Select::new("How do you want to plan your weekly menu?", options).prompt().unwrap();
        match ans {
            "Use Smart Points" => {
                generate_weekly_menu_using_smart_points(account);
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