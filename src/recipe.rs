use core::fmt;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub enum RecipeError {
    ReadError(std::io::Error),
    ParseError(serde_json::Error),
}

impl fmt::Display for RecipeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RecipeError::ReadError(err) => write!(f, "ReadError: {}", err),
            RecipeError::ParseError(err) => write!(f, "ParseError: {}", err),
        }
    }
}

impl Error for RecipeError {}

impl From<std::io::Error> for RecipeError {
    fn from(error: std::io::Error) -> Self {
        RecipeError::ReadError(error)
    }
}

impl From<serde_json::Error> for RecipeError {
    fn from(error: serde_json::Error) -> Self {
        RecipeError::ParseError(error)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Nutrition {
    #[serde(rename = "type")]
    pub nutrient: String,
    pub amount: f32,
    pub unit: String,
}

impl Nutrition {
    #[allow(dead_code)]
    pub fn print(&self) {
        println!("{}: {} {}", self.nutrient, self.amount, self.unit);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Time {
    pub prep: f32,
    pub total: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ingredient {
    pub quantity: f32,
    pub unit: String,
    pub name: String,
    #[serde(default)]
    pub prep: String,
    #[serde(rename = "type")]
    pub category: String,
}

impl Ingredient {
    #[allow(dead_code)]
    pub fn print(&self) {
        println!(
            "{} {} {} {}",
            self.quantity, self.unit, self.prep, self.name
        );
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Recipe {
    pub name: String,
    pub servings: u8,
    #[serde(rename = "points-per-serving")]
    pub points_per_serving: u8,
    pub ingredients: Vec<Ingredient>,
    pub steps: Vec<String>,
    #[serde(rename = "nutrition")]
    pub nutrients: Vec<Nutrition>,
    #[serde(rename = "imageURL")]
    pub image_url: String,
    #[serde(rename = "originalURL")]
    pub recipe_source: String,
}

impl Recipe {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Recipe, RecipeError> {
        println!("Reading: {}", path.as_ref().to_string_lossy());
        let file_data = fs::read_to_string(&path)?;
        let recipe = serde_json::from_str(&file_data)?;

        Ok(recipe)
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        println!("{}", self.name);
        println!("Servings: {}", self.servings);
        println!("Points: {}", self.points_per_serving);
        println!();

        println!("[Ingredients]");
        for ingredient in &self.ingredients {
            ingredient.print();
        }
        println!();

        println!("[Steps]");
        for step in &self.steps {
            println!("{}", step)
        }
        println!();

        println!("[Nutrition Info]");
        for nutrient in &self.nutrients {
            nutrient.print();
        }
        println!();
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use rstest::rstest;

// #[rstest]
// #[case("nyms-recipes/recipes/fathead-pizza.json")]
// fn test_recipe_read<P: AsRef<Path>>(#[case] file: P) {
//     assert!(Recipe::new(file).is_ok());
// }

// #[rstest]
// #[case("nyms-recipes/recipes/fathead-pizza.json")]
// fn test_recipe_parse<P: AsRef<Path>>(#[case] file: P) {
//     match Recipe::new(file) {
//         Ok(recipe) => println!("{}", recipe.name),
//         Err(err) => println!("Error: {}", err),
//     };
// }
//}
