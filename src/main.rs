use crate::recipe::Recipe;
use std::io;
use std::path::{ Path, PathBuf };
use std::error::Error;
use std::fs;
use inquire::*;

mod recipe;

// #[derive(Debug)]
// pub enum RecipeError {
//     ReadError(std::io::Error),
//     ParseError(serde_json::Error)
// }

// impl fmt::Display for RecipeError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             RecipeError::ReadError(err) => write!(f, "ReadError: {}", err),
//             RecipeError::ParseError(err) => write!(f, "ParseError: {}", err)
//         }
//     }
// }

// impl Error for RecipeError {}

// impl From<std::io::Error> for RecipeError {
//     fn from(error: std::io::Error) -> Self {
//         RecipeError::ReadError(error)
//     }
// }

// impl From<serde_json::Error> for RecipeError {
//     fn from(error: serde_json::Error) -> Self {
//         RecipeError::ParseError(error)
//     }
// }

#[allow(dead_code)]
fn lbs_to_ounces(lbs: f32) -> f32 {
    lbs * 16.0
}

#[allow(dead_code)]
fn ounces_to_lbs(oz: f32) -> f32 {
    oz / 16.0
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
        },
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

mod MAIN_MENU {
    pub const PLAN_WEEK: &str = "Plan week";
    pub const PLAN_DAY: &str = "Plan day";
}

fn main() -> std::io::Result<()> {
    // TODO(mdeforge): Get rid of this unwrap
    let read_result = read_recipes("nyms-recipes/recipes").unwrap();

    mod MainMenu {
        pub enum Options {
            WEEKLY,
            DAILY
        }

        impl Options {
            fn as_str(&self) -> &'static str {
                match self {
                    Options::WEEKLY => "Plan meals for a week",
                    Options::DAILY => "Plan meals for a day"
                }
            }
        }

        static OPTIONS: Vec<Options> = vec!{ self::Options::WEEKLY, self::Options::DAILY };
    }


    //let options: Vec<_> = MainMenuOptions::into();
    let ans: Result<&str, InquireError> = Select::new("What would you like to do?", options).prompt();

    match ans {
        Ok(choice) => {
            println!("{}! That's my choice too!", choice);
            if choice == "Generate Weekly Meal Plan" {
                read_recipes("nyms-recipes/recipes").unwrap()
                    .iter()
                    .for_each(|recipe| recipe.print());
            }
        },
        Err(_) => println!("There was an error, please try again.")
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use assert_fs::{prelude::*, TempDir};

    #[test]
    fn test_get_files_in_directory() {
        let temp_dir = TempDir::new().unwrap();
        let new_dir = temp_dir.child("test");
        let file1 = new_dir.child("test1.txt");
        let file2 = new_dir.child("test2.txt");
        file1.touch().unwrap();
        file2.touch().unwrap();

        let files = get_files_in_directory(new_dir.path()).unwrap();
        assert_eq!(Path::new(new_dir.path()).join("test1.txt"), files[0].as_path());
        assert_eq!(Path::new(new_dir.path()).join("test2.txt"), files[1].as_path());

        temp_dir.close().unwrap();
    }

    #[rstest]
    #[case("nyms-recipes/recipes/fathead-pizza.json")]
    fn test_read_recipe(#[case] file: &str) {
        let recipe: Recipe = read_recipe(Path::new(file)).unwrap();
        assert_eq!("Fathead Pizza", recipe.name);
        assert_eq!(4, recipe.servings);
        assert_eq!(12, recipe.ingredients.len());
        assert_eq!("cheese", recipe.ingredients[0].category);
    }

    //#[rstest]
    //#[case()]
}
