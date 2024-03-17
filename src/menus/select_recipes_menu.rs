use crate::user::User;
//use crate::person::Person;
use super::menu::Menu;
//use super::main_menu::MainMenu;
//use inquire::{Select, Text, CustomType};

#[derive(Default)]
pub struct SelectRecipesMenu;

// match user.read_recipes("./nyms-recipes") {
//     Ok(()) => {
//         println!("Successfully read recipes from ./nyms-recipes");

//         let options = user.get_recipe_names();
//         let ans = MultiSelect::new("Choose", options).prompt();
//         match ans {
//             Ok(_) => {
//                 println!("Recipes added");
//                 Some(Box::new(MainMenu::default()))
//             },
//             Err(_) => {
//                 println!("Error");
//                 Some(Box::new(MainMenu::default()))
//             }
//         }
//     },
//     Err(err) => { 
//         println!("Failed to read recipes: {err}")
//         Some(Box::new(MainMenu::default()))
//     }
// }

impl Menu for SelectRecipesMenu {
    fn prompt(&self, _account: &mut User) -> Option<Box<dyn Menu>> {
        todo!("Select recipes!")
    }
}