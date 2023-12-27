use crate::recipe::Recipe;
use std::io;
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Recipes {
    recipes: Vec<Recipe>
}

impl Recipes {
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
        // Gather recipe files
        let recipe_files = self.get_files_in_directory(folder)?;

        // Read recipes into vector
        self.recipes = recipe_files
            .into_iter()
            .filter_map(|recipe_file| Some(Recipe::new(recipe_file).unwrap()))
            .collect();
        
        // Sort recipes by smart points
        self.recipes.sort_by_key(|x| x.points_per_serving);

        println!("Number of recipes: {}", self.recipes.len());

        Ok(())
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