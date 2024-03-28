use std::path::Path;

use crate::account::Account;
use crate::user::User;
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

// TODO(mdeforge): Would love an option to validate recipe box. Workaround now is to
//                 edit recipe box location, even if it's the same, to get free check.

impl Menu for SetupMenu {
    fn prompt(&self, _account: &mut Account) -> Option<Box<dyn Menu>> {
        let options = vec!["Add person", "Remove person", "Configure Person", "Edit recipe box location", "Back"];
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
    fn prompt(&self, account: &mut Account) -> Option<Box<dyn Menu>> {
        let name = Text::new("Please enter a name:").prompt().unwrap();

        // Check if empty
        if name.is_empty() {
            println!("Name cannot be empty, please enter a valid name.");
            return Some(Box::new(AddPersonMenu::default()));
        }

        // Check if it exists
        if account.has_user(name) {
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

        // Add user
        let user: User = User::new(name.clone(), daily_points.unwrap(), extra_points.unwrap());
        account.add_user(user).unwrap();
        
        println!("{} has been added to the account.", name.clone());

        Some(Box::new(SetupMenu::default()))
    }
}

impl Menu for RemovePersonMenu {
    fn prompt(&self, account: &mut Account) -> Option<Box<dyn Menu>> {
        let mut options = account.get_users();
        options.push(String::from("Cancel"));

        let user = Select::new("Select user to remove:", options)
            .prompt()
            .unwrap();

        let user_str = user.as_str();
        match user_str {
            "Cancel" => return Some(Box::new(SetupMenu::default())),
            _ => {
                account.remove_user(&user);
                return Some(Box::new(SetupMenu::default()));
            }
        }
    }
}

impl Menu for ConfigPersonMenu {
    fn prompt(&self, account: &mut Account) -> Option<Box<dyn Menu>> {
        // TODO(mdeforge): Add back option at this point so that they don't have to go to the next menu
        // TODO(mdeforge): Back should take them to the person select menu, not all the way back

        let people = account.get_users();
        let selection = Select::new("Which person do you want to configure?", people)
            .prompt()
            .unwrap();

        // Get user
        let user: &mut User = account.get_user(selection)?;

        let options = vec!["Select favorite recipes", "Adjust smart points", "Back"];
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

                let recipe_names = account.recipe_box().recipe_names();
                
                let indices = user.get_indices_of_favorites(&recipe_names);
                let favorites_result = MultiSelect::new("Select favorite recipes", recipe_names)
                    .with_default(&indices)
                    .prompt();

                match favorites_result {
                    Ok(favorites) => {
                        user.set_favorites(favorites);
                        account.save().unwrap_or_else(|err| {
                            println!("Failed to save user: {}", err);
                            Some(Box::new(MainMenu::default()));
                        });
                        
                        return Some(Box::new(MainMenu::default()))
                    },
                    Err(_) => {
                        println!("Could not find any recipes to favorite.");
                        return Some(Box::new(MainMenu::default()))
                    }
                }
            },
            "Adjust smart points" => {
                // Get daily points
                let daily_point_text = format!("How many points does {} get each day?", user.get_name());
                let daily_points = CustomType::<i32>::new(&daily_point_text)
                    .with_formatter(&|i| format!("{}", i))
                    .with_error_message("Please type a valid number")
                    .with_help_message("Type the amount of Smart Points the person gets per day")
                    .prompt();
            
                // Get extra points
                let extra_points_text = format!("How many extra points does {} get each week?", user.get_name());
                let extra_points = CustomType::<i32>::new(&extra_points_text)
                    .with_formatter(&|i| format!("{}", i))
                    .with_error_message("Please type a valid number")
                    .with_help_message("Type the amount of extra Smart Points the person gets per week")
                    .prompt();

                match account.save() {
                    Ok(_) => {
                        println!("Successfully saved favorites.");
                        return Some(Box::new(MainMenu::default()))
                    },
                    Err(err) => {
                        println!("Failed to save favorites: {}", err);
                        return Some(Box::new(MainMenu::default()))
                    }
                }
            },
            "Back" => Some(Box::new(MainMenu::default())),
            _ => None,
        }
    }
}

impl Menu for EditRecipeBoxMenu {
    fn prompt(&self, account: &mut Account) -> Option<Box<dyn Menu>> {
        let user_path = Text::new("Please enter the path to recipe folder:")
            .with_initial_value(&account.recipe_path())
            .prompt()
            .unwrap();

        let recipe_path = Path::new(user_path.as_str());
        if recipe_path.exists() {
            if account.recipe_box().read_recipes(recipe_path).is_ok() {
                match account.set_recipe_path(user_path) {
                    Ok(_) => {
                        println!("Updated recipe path.");
                        return Some(Box::new(SetupMenu::default()))
                    },
                    Err(_) => {
                        println!("Failed to update recipe path.");
                        return Some(Box::new(SetupMenu::default()))
                    }
                }
            } else {
                println!("Could not find a valid recipe folder.");
                return Some(Box::new(EditRecipeBoxMenu::default()));    
            }
        } else {
            println!("Path does not exist, please enter a valid path.");

            return Some(Box::new(EditRecipeBoxMenu::default()));
        }
    }
}