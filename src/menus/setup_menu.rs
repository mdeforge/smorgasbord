use crate::user::User;
use crate::person::Person;
use super::menu::Menu;
use super::main_menu::MainMenu;
use inquire::{Select, Text};

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

        // let point_text = format!("How many points does {} have each day?", name);
        // let points = Text::new(&point_text).prompt().unwrap();
        // if points.parse() {
        //     Ok(points) => user.add_person(&name, Person::default());
        // }

        
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
                if user.remove_person(&ans).is_some() {
                    println!("{} successfully removed from the account.", name);
                }

                return Some(Box::new(SetupMenu::default()));
            }
        }
    }
}