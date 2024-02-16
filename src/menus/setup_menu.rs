use std::path::Path;

use crate::user::User;
use crate::person::Person;
use super::menu::Menu;
use super::main_menu::MainMenu;
use inquire::{Select, Text, CustomType, MultiSelect};

#[derive(Default)]
pub struct SetupMenu;

#[derive(Default)]
struct RemovePersonMenu;

#[derive(Default)]
struct AddPersonMenu;

#[derive(Default)]
pub struct ConfigPersonMenu;
#[derive(Default)]
pub struct EditRecipeBoxMenu;

impl Menu for SetupMenu {
    fn prompt(&self, _user: &mut User) -> Option<Box<dyn Menu>> {
        let options = vec!["Add person", "Remove person", "Add recipe location", "Remove recipe location", "Back"];
        let ans = Select::new("Choose", options).prompt().unwrap();
        match ans {
            "Add person" => Some(Box::new(AddPersonMenu::default())),
            "Remove person" => Some(Box::new(RemovePersonMenu::default())),
            "Configure Person" => Some(Box::new(ConfigPersonMenu::default())),
            "Edit recipe box location" => Some(Box::new(EditRecipeBoxMenu::default())),
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

impl Menu for ConfigPersonMenu {
    fn prompt(&self, user: &mut User) -> Option<Box<dyn Menu>> {
        // TODO(mdeforge): Add back option at this point so that they don't have to go to the next menu
        // TODO(mdeforge): Back should take them to the person select menu, not all the way back

        let options = vec!["Select favorite recipes", "Adjust daily points", "Adjust extra points", "Back"];
        let ans = Select::new("What do you wish to configure?", options)
            .prompt()
            .unwrap();
        
        match ans {
            "Select favorite recipes" => {
                // NOTE(mdeforge): Multi-select needs a list of names and a list of indices of what 
                //                 should be selected by default. It will give you a string of what's
                //                 been selected.
                
                // NOTE(mdeforge): We want to save favorites as strings and only convert them to
                //                 indices when needed.

                // NOTE(mdeforge): Since recipe's are account bound but favorites are per person, we have to
                //                 piece the info together.

                let people = user.get_people();
                let selection = Select::new("Which person do you want to configure?", people)
                    .prompt()
                    .unwrap();
                
                let recipe_names = user.recipe_box().recipe_names();
                
                if let Some(person) = user.find_person(selection) {
                    let indices = person.get_indices_of_favorites(&recipe_names);
                        
                    let favorites = MultiSelect::new("Select favorite recipes", recipe_names)
                        .with_default(&indices)
                        .prompt()
                        .unwrap();
    
                    person.set_favorites(favorites);
                }

                return Some(Box::new(MainMenu::default()))
            },
            "Adjust daily points" => {
                return Some(Box::new(MainMenu::default()))
            },
            "Adjust extra points" =>  {
                return Some(Box::new(MainMenu::default()))
            },
            "Back" => Some(Box::new(MainMenu::default())),
            _ => None,
        }
    }
}

impl Menu for EditRecipeBoxMenu {
    fn prompt(&self, user: &mut User) -> Option<Box<dyn Menu>> {
        let user_path = Text::new("Please enter the path to recipe folder.")
            .with_initial_value(&user.recipe_path())
            .prompt()
            .unwrap();

        let recipe_path = Path::new(user_path.as_str());
        if recipe_path.exists() {
            // TODO(mdeforge): Or what?
            if user.recipe_box().read_recipes(recipe_path).is_ok() {
                user.set_recipe_path(user_path);
            }

            return Some(Box::new(SetupMenu::default()))
        } else {
            println!("Path does not exist, please enter a valid path.");

            return Some(Box::new(EditRecipeBoxMenu::default()));
        }
    }
}