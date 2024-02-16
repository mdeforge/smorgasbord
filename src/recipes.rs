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
    fn get_files_in_directory<P: AsRef<Path>>(&self, folder: P) -> Result<Vec<PathBuf>, io::Error> {
        // Get contents of directory
        let entries = fs::read_dir(folder)?;
    
        // Filter out everything but files
        let file_path: Vec<PathBuf> = entries
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().map_or(false, |ft| ft.is_file()))
            .map(|entry| entry.path())
            .collect();

        // TODO(mdeforge): Only find json files?
    
        Ok(file_path)
    }

    pub fn read_recipes<P: AsRef<Path>>(&mut self, folder: P) -> Result<(), io::Error> {
        // Clear existing recipes
        self.recipes.clear();

        // Gather recipe files
        let recipe_files = self.get_files_in_directory(folder)?;

        // Read recipes into vector
        self.recipes = recipe_files
            .into_iter()
            .filter_map(|recipe_file| Some(Recipe::new(recipe_file).unwrap()))
            .collect();
        
        // Sort recipes by smart points which will be useful later when generating meals
        self.recipes.sort_by_key(|x| x.points_per_serving);

        println!("Number of recipes: {}", self.recipes.len());

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