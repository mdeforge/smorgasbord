use core::fmt;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug)]
pub enum UserError {
    ReadError(std::io::Error),
    WriteError(std::io::Error),
    ParseError(serde_json::Error),
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserError::ReadError(err) => write!(f, "ReadError: {}", err),
            UserError::WriteError(err) => write!(f, "WriteError: {}", err),
            UserError::ParseError(err) => write!(f, "ParseError: {}", err),
        }
    }
}

impl Error for UserError {}

impl From<std::io::Error> for UserError {
    fn from(error: std::io::Error) -> Self {
        UserError::ReadError(error)
    }
}

// impl From<std::io::Error> for PersonError {
//     fn from(error: std::io::Error) -> Self {
//         PersonError::WriteError(error)
//     }
// }

impl From<serde_json::Error> for UserError {
    fn from(error: serde_json::Error) -> Self {
        UserError::ParseError(error)
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct User {
    name: String,
    pub daily_points: i32,
    pub extra_points: i32,
    favorite_recipes: Vec<String>
}

impl User {
    pub fn new(name: String, daily_points: i32, extra_points: i32) -> Self {
        Self {
            name: name,
            daily_points: daily_points,
            extra_points: extra_points,
            favorite_recipes: Vec::new()
        }
    }

    pub fn get_name(&self) -> String {
        self.name
    }

    pub fn set_daily_smart_point_limit(&mut self, limit: i32) {
        self.daily_points = limit;
    }

    // Compares the current list of favorites with the recipe box to come up
    // with a list of items that match.
    pub fn get_indices_of_favorites(&self, recipe_names: &Vec<String>) -> Vec<usize> {
        recipe_names
            .iter()
            .filter_map(|favorite| recipe_names.iter().position(|r| r == favorite))
            .collect()
    }

    pub fn set_favorites(&mut self, favorites: Vec<String>) {
        self.favorite_recipes = favorites;
    }
}