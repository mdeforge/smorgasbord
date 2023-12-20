use inquire::{Select, Text};
use std::error::Error;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use crate::data::data::USER_DATA;
use crate::person::Person;
use crate::recipe::Recipe;
use crate::user::User;
//use crate::util::Util;

// TODO(mdeforge): Need to add support for tracking two people's smart point values
// TODO(mdeforge): Replace unwrap's after prompt with error handling

pub trait Menu {
    fn prompt(&self) -> Option<Box<dyn Menu>>;
}

#[derive(Default)]
pub struct MainMenu;

#[derive(Default)]
struct SetupMenu;

#[derive(Default)]
struct AddPersonMenu;

#[derive(Default)]
struct RemovePersonMenu;

#[derive(Default)]
struct WeeklyMenu;

#[derive(Default)]
struct DailyMenu;

fn find_recipes_using_smart_points() {
    let mut recipes = read_recipes("nyms-recipes/recipes").unwrap();

    // Sort recipes by points per serving
    recipes.sort_by_key(|x| x.points_per_serving);

    println!("Smart Points!");
}

fn find_recipes_randomly() {
    println!("Randomly!")
}

fn get_files_in_directory<P: AsRef<Path>>(folder: P) -> Result<Vec<PathBuf>, io::Error> {
    let entries = fs::read_dir(folder)?;

    let file_path: Vec<PathBuf> = entries
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().map_or(false, |ft| ft.is_file()))
        .map(|entry| entry.path())
        .collect();

    Ok(file_path)
}

fn read_recipe<P: AsRef<Path>>(recipe_file: P) -> Option<Recipe> {
    let file_as_string = recipe_file.as_ref().to_string_lossy().into_owned();
    let result = Recipe::new(recipe_file);

    match result {
        Ok(recipe) => {
            println!("Read: {:?}", file_as_string);
            return Some(recipe);
        }
        Err(err) => {
            eprintln!("Failed to read: {}", file_as_string);
            eprintln!("-- {}", err);
            return None;
        }
    }
}

fn read_recipes(folder: &str) -> Result<Vec<Recipe>, Box<dyn Error>> {
    let recipe_files = get_files_in_directory(folder)?;

    let recipes: Vec<_> = recipe_files
        .into_iter()
        .filter_map(|recipe_file| read_recipe(recipe_file))
        .collect();

    println!("Number of recipes: {}", recipes.len());

    Ok(recipes)
}

impl Menu for MainMenu {
    fn prompt(&self) -> Option<Box<dyn Menu>> {
        let options = vec!["Create weekly meal plan", "Create daily meal plan", "Exit"];
        let ans = Select::new("Choose", options).prompt().unwrap();
        match ans {
            "Create weekly meal plan" => Some(Box::new(WeeklyMenu::default())),
            "Create daily meal plan" => Some(Box::new(DailyMenu::default())),
            "Exit" => None,
            _ => None
        }
    }
}

impl Menu for SetupMenu {
    fn prompt(&self) -> Option<Box<dyn Menu>> {
        let options = vec!["Add person", "Remove person"];
        let ans = Select::new("Choose", options).prompt().unwrap();
        match ans {
            "Add person" => Some(Box::new(WeeklyMenu::default())),
            "Remove person" => Some(Box::new(DailyMenu::default())),
            "Main Menu" => Some(Box::new(MainMenu::default())),
            _ => None
        }
    }
}

impl Menu for AddPersonMenu {
    fn prompt(&self) -> Option<Box<dyn Menu>> {
        let name = Text::new("Please enter a name:").prompt().unwrap();

        // Check if empty
        if name.is_empty() {
            println!("Name cannot be empty");
            return Some(Box::new(AddPersonMenu::default()));
        }

        // Check if it exists
        if USER_DATA.has_person(&name) {
            println!("Person {} already exists, please use another name", name);
            return Some(Box::new(AddPersonMenu::default()));
        }

        USER_DATA.add_person(name, Person::default());
        Some(Box::new(SetupMenu::default()))
    }
}

impl Menu for RemovePersonMenu {
    fn prompt(&self) -> Option<Box<dyn Menu>> {
        let options = vec!["Add person", "Remove person"];
        let ans = Select::new("Choose", options).prompt().unwrap();
        match ans {
            "Add person" => Some(Box::new(WeeklyMenu::default())),
            "Remove person" => Some(Box::new(DailyMenu::default())),
            "Main Menu" => Some(Box::new(MainMenu::default())),
            _ => None
        }
    }
}

impl Menu for WeeklyMenu {
    fn prompt(&self) -> Option<Box<dyn Menu>> {
        let options = vec!["Use Smart Points", "Just Randomize", "Main Menu"];
        let ans = Select::new("Choose", options).prompt().unwrap();
        match ans {
            "Use Smart Points" => {
                find_recipes_using_smart_points();
                Some(Box::new(MainMenu::default()))
            },
            "Just Randomize" => {
                find_recipes_randomly();
                Some(Box::new(MainMenu::default()))
            },
            "Main Menu" => Some(Box::new(MainMenu::default())),
            _ => None
        }
    }
}

impl Menu for DailyMenu {
    fn prompt(&self) -> Option<Box<dyn Menu>> {
        let options = vec!["Use Smart Points", "Just Randomize", "Main Menu"];
        let ans = Select::new("Choose", options).prompt().unwrap();
        match ans {
            "Use Smart Points" => {
                find_recipes_using_smart_points();
                Some(Box::new(MainMenu::default()))
            },
            "Just Randomize" => {
                find_recipes_randomly();
                Some(Box::new(MainMenu::default()))
            },
            "Main Menu" => Some(Box::new(MainMenu::default())),
            _ => None
        }
    }
}
