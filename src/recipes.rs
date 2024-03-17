use crate::recipe::Recipe;
use std::io;
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct RecipeBox {
    recipes: Vec<Recipe>
}

impl RecipeBox {
    fn get_recipes_in_directory<P: AsRef<Path>>(&self, folder: P) -> Result<Vec<PathBuf>, io::Error> {
        // Get contents of directory
        let entries = fs::read_dir(folder)?;
    
        // Filter out everything but json files
        let file_path: Vec<PathBuf> = entries
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                let file_type = match entry.file_type() {
                    Ok(ft) => ft,
                    Err(_) => return false, // or handle the error as appropriate
                };
                file_type.is_file() && entry.path().extension().map_or(false, |ext| ext == "json")
            })
            .map(|entry| entry.path())
            .collect();

        Ok(file_path)
    }

    pub fn read_recipes<P: AsRef<Path>>(&mut self, folder: P) -> Result<(), io::Error> {
        // Clear existing recipes
        self.recipes.clear();

        // Gather recipe files
        let recipe_files = self.get_recipes_in_directory(folder)?;
        for recipe in recipe_files.clone() {
            println!("{}", recipe.to_string_lossy());
        }

        // Read recipes into vector
        // TODO(mdeforge): Maybe only read valid ones? Print invalid?
        self.recipes = recipe_files
            .into_iter()
            .filter_map(|recipe_file| Some(Recipe::new(recipe_file).unwrap()))
            .collect();
        
        // Sort recipes by smart points which will be useful later when generating meals
        self.recipes.sort_by_key(|x| x.points_per_serving);

        println!("Number of recipes found: {}", self.recipes.len());

        Ok(())
    }

    pub fn recipe_names(&self) -> Vec<String> {
        self.recipes
            .iter()
            .map(|recipe| recipe.name.to_owned())
            .collect()
    }

    pub fn generate_daily_menu_using_smart_points() {
        println!("Daily smart points!");
    }

    pub fn generate_weekly_menu_using_smart_points() {
        println!("Weekly smart points!")
    }
    
    pub fn generate_daily_menu_randomly() {
        println!("Daily randomly!")
    }

    pub fn generate_weekly_menu_randomly() {
        println!("Weekly randomly!")
    }
}