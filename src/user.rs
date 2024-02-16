use core::fmt;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub enum PersonError {
    ReadError(std::io::Error),
    WriteError(std::io::Error),
    ParseError(serde_json::Error),
}

impl fmt::Display for PersonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PersonError::ReadError(err) => write!(f, "ReadError: {}", err),
            PersonError::WriteError(err) => write!(f, "WriteError: {}", err),
            PersonError::ParseError(err) => write!(f, "ParseError: {}", err),
        }
    }
}

impl Error for PersonError {}

impl From<std::io::Error> for PersonError {
    fn from(error: std::io::Error) -> Self {
        PersonError::ReadError(error)
    }
}

// impl From<std::io::Error> for PersonError {
//     fn from(error: std::io::Error) -> Self {
//         PersonError::WriteError(error)
//     }
// }

impl From<serde_json::Error> for PersonError {
    fn from(error: serde_json::Error) -> Self {
        PersonError::ParseError(error)
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct User {
    pub daily_points: i32,
    pub extra_points: i32,
    favorite_recipes: Vec<String>
}

impl User {
    pub fn new(daily_points: i32, extra_points: i32) -> Self {
        Self {
            daily_points: daily_points,
            extra_points: extra_points,
            favorite_recipes: Vec::new()
        }
    }
    pub fn load<P: AsRef<Path>>(path: P) -> Result<User, PersonError> {
        let file_data: String = fs::read_to_string(&path)?;
        let person = serde_json::from_str(&file_data)?;

        Ok(person)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn Error>> {
        let file = fs::File::create(path)?;
        serde_json::to_writer(file, &self)?;

        Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::*;
    use rstest::rstest;

    #[rstest]
    #[case("person.json")]
    fn test_person_save_and_load<P: AsRef<Path>>(#[case] file: P) {
        let temp_dir = TempDir::new().unwrap();
        let filename = temp_dir.join(file);

        let mut save_person = User::default();
        save_person.set_daily_smart_point_limit(50);

        let save_result = save_person.save(&filename);
        assert!(save_result.is_ok());

        let result = User::load(&filename);
        assert!(result.is_ok());

        let load_person = result.unwrap();
        assert_eq!(load_person.daily_points, 50);
    }
}
