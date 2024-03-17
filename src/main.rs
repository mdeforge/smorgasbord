mod user;
mod recipe;
mod recipes;
mod account;
mod menus;

use account::*;
use menus::menu::*;
use menus::main_menu::MainMenu;
use inquire::Select;
use std::path::{Path, PathBuf};
use std::fs;
use std::ffi::OsStr;

use crate::menus::new_user_menu::NewUserMenu;

// TODO(mdeforge): Need to add support for tracking two people's smart point values
// TODO(mdeforge): Replace unwrap's after prompt with error handling
// TODO(mdeforge): We can move the actual inquire prompt out of the prompt() function (or to it's own function)
//                 so that we can better unit test what some of the menus should be doing.

fn get_user_files_in_directory<P: AsRef<Path>>(path: P) -> Option<Vec<PathBuf>> {
    match fs::read_dir(&path) {
        Ok(entries) => {
            let files: Vec<_> = entries
                .filter_map(|entry| entry.ok())
                .filter(|entry| entry.file_type().map_or(false, |ft| ft.is_file()))
                .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "user"))
                .map(|entry| entry.path())
                .collect();

            if files.len() > 0 {
                Some(files)
            } else {
                None
            }
        },
        Err(err) => {
            println!("Failed to read directory: {}", err);
            None
        }
    }
}

fn load_user_file<P: AsRef<Path>>(file: P) -> Option<Account> {
    match Account::load(file) {
        Ok(user) => Some(user),
        Err(err) => {
            println!("Failed to load user. {}", err);

            None
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut _current_menu: Option<Box<dyn Menu>> = Some(Box::new(MainMenu::default()));
    let mut user = Account::default();

    let path = Path::new("./data");
    fs::create_dir_all(&path).expect("Failed to create data directory!");
    
    match get_user_files_in_directory(&path) {
        Some(files) => {
            if files.len() > 1 {
                let options: Vec<_> = files
                    .iter()
                    .map(|path_buf| path_buf.file_name().and_then(OsStr::to_str).unwrap())
                    .collect();
    
                let choice = 
                    Select::new("Select user file to load:", options).prompt().unwrap();
    
                user = load_user_file(choice).unwrap_or_else(|| {
                    _current_menu = Some(Box::new(NewUserMenu::default()));
                    Account::default() // Create new user
                });
            } else {
                user = load_user_file(&files[0]).unwrap_or_else(|| {
                    _current_menu = Some(Box::new(NewUserMenu::default()));
                    Account::default() // Create new user
                });
            }

            println!("Welcome back, {}!", user.name);
        },
        None => {
            println!("Entering account setup.");
            _current_menu = Some(Box::new(NewUserMenu::default()));
        }
    };

    while _current_menu.is_some() {
        _current_menu = _current_menu.unwrap().prompt(&mut user);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::{prelude::*, TempDir};
    use std::ffi::OsStr;

    #[test]
    fn test_get_user_files_in_directory() {
        let temp_dir = TempDir::new().unwrap();
        let _s = temp_dir.as_ref().to_str().unwrap();
        let file1 = temp_dir.child("main.user");
        let file2 = temp_dir.child("other.txt");
        file1.touch().unwrap();
        file2.touch().unwrap();

        match get_user_files_in_directory(&temp_dir) {
            Some(files) => {
                assert_eq!(files.len(), 1);
                let filename = 
                    files[0].file_name().and_then(OsStr::to_str).unwrap();
                assert_eq!(filename, "main.user"); 
            },
            None => assert!(false, "Expected files")
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use assert_fs::{prelude::*, TempDir};
//     use rstest::rstest;

//     #[test]
//     fn test_get_files_in_directory() {
//         let temp_dir = TempDir::new().unwrap();
//         let new_dir = temp_dir.child("test");
//         let file1 = new_dir.child("test1.txt");
//         let file2 = new_dir.child("test2.txt");
//         file1.touch().unwrap();
//         file2.touch().unwrap();

//         let files = get_files_in_directory(new_dir.path()).unwrap();
//         assert_eq!(
//             Path::new(new_dir.path()).join("test1.txt"),
//             files[0].as_path()
//         );
//         assert_eq!(
//             Path::new(new_dir.path()).join("test2.txt"),
//             files[1].as_path()
//         );

//         temp_dir.close().unwrap();
//     }

//     #[rstest]
//     #[case("nyms-recipes/recipes/fathead-pizza.json")]
//     fn test_read_recipe(#[case] file: &str) {
//         let recipe: Recipe = read_recipe(Path::new(file)).unwrap();
//         assert_eq!("Fathead Pizza", recipe.name);
//         assert_eq!(4, recipe.servings);
//         assert_eq!(12, recipe.ingredients.len());
//         assert_eq!("cheese", recipe.ingredients[0].category);
//     }

//     //#[rstest]
//     //#[case()]
// }
