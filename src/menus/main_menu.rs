use crate::Account;
use super::setup_menu::SetupMenu;
use super::daily_menu::DailyMenu;
use super::weekly_menu::WeeklyMenu;
use super::menu::Menu;
use inquire::Select;

#[derive(Default)]
pub struct MainMenu;

impl Menu for MainMenu {
    fn prompt(&self, _account: &mut Account) -> Option<Box<dyn Menu>> {
        let options = vec![
            "Create weekly meal plan",
            "Create daily meal plan",
            "Account Setup",
            "Exit",
        ];

        let ans = Select::new("What would you like to do?", options)
            .prompt()
            .unwrap();

        match ans {
            "Create weekly meal plan" => Some(Box::new(WeeklyMenu::default())),
            "Create daily meal plan" => Some(Box::new(DailyMenu::default())),
            "Account Setup" => Some(Box::new(SetupMenu::default())),
            "Exit" => None,
            _ => None,
        }
    }
}