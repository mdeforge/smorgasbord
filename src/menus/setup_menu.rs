use crate::user::User;
use crate::person::Person;
use super::menu::Menu;
use super::main_menu::MainMenu;
use inquire::{Select, Text, CustomType};

#[derive(Default)]
pub struct SetupMenu;

#[derive(Default)]
struct RemovePersonMenu;

#[derive(Default)]
struct AddPersonMenu;

impl Menu for SetupMenu {
    fn prompt(&self, user: &mut User) -> Option<Box<dyn Menu>> {
        let options = vec!["Add person", "Remove person", "Back"];
        let ans = Select::new("Choose", options).prompt().unwrap();
        match ans {
            "Add person" => Some(Box::new(AddPersonMenu::default())),
            "Remove person" => Some(Box::new(RemovePersonMenu::default())),
            "Back" => Some(Box::new(MainMenu::default())),
            _ => None,
        }
    }
}

impl Menu for AddPersonMenu {
    // TODO(mdeforge): How to cancel?
    fn prompt(&self, user: &mut User) -> Option<Box<dyn Menu>> {
        let name = Text::new("Please enter a name:").prompt().unwrap();

        // Check if empty
        if name.is_empty() {
            println!("Name cannot be empty, please enter a valid name.");
            return Some(Box::new(AddPersonMenu::default()));
        }

        // Check if it exists
        if user.has_person(&name) {
            println!("Person {} already exists, please use another name.", name);
            return Some(Box::new(AddPersonMenu::default()));
        }

        // Get daily points
        let daily_point_text = format!("How many points does {} get each day?", name);
        let daily_points = CustomType::<i32>::new(&daily_point_text)
            .with_formatter(&|i| format!("{}", i))
            .with_error_message("Please type a valid number")
            .with_help_message("Type the amount of Smart Points the person gets per day")
            .prompt();
    
        // Get extra points
        let extra_points_text = format!("How many extra points does {} get each week?", name);
        let extra_points = CustomType::<i32>::new(&extra_points_text)
            .with_formatter(&|i| format!("{}", i))
            .with_error_message("Please type a valid number")
            .with_help_message("Type the amount of extra Smart Points the person gets per week")
            .prompt();

        // Get extra points
        let person: Person = Person::new(daily_points.unwrap(), extra_points.unwrap());
        user.add_person(&name, person).unwrap();
        
        println!("{} has been added to the account.", name);

        Some(Box::new(SetupMenu::default()))
    }
}

impl Menu for RemovePersonMenu {
    fn prompt(&self, user: &mut User) -> Option<Box<dyn Menu>> {
        let mut options = user.get_people();
        options.push(String::from("Cancel"));

        let ans = Select::new("Select person to remove:", options)
            .prompt()
            .unwrap();
        let name = ans.as_str();
        match name {
            "Cancel" => return Some(Box::new(SetupMenu::default())),
            _ => {
                if user.remove_person(&ans).is_ok() {
                    println!("{} successfully removed from the account.", name);
                }

                return Some(Box::new(SetupMenu::default()));
            }
        }
    }
}