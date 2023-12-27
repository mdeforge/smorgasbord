use user::User;

mod person;
mod recipe;
mod recipes;
mod user;
mod menus;

use menus::menu::*;
use menus::main_menu::MainMenu;

// TODO(mdeforge): Need to add support for tracking two people's smart point values
// TODO(mdeforge): Replace unwrap's after prompt with error handling
// TODO(mdeforge): We can move the actual inquire prompt out of the prompt() function (or to it's own function)
//                 so that we can better unit test what some of the menus should be doing.



fn main() -> std::io::Result<()> {
    // TODO(mdeforge): Get rid of this unwrap
    //let read_result = read_recipes("nyms-recipes/recipes").unwrap();
    let mut user = User::default();

    let mut _current_menu: Option<Box<dyn Menu>> = Some(Box::new(MainMenu::default()));
    while _current_menu.is_some() {
        _current_menu = _current_menu.unwrap().prompt(&mut user);
    }

    Ok(())
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
