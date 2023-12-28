use crate::person::Person;
use crate::recipes::Recipes;
use serde::{Deserialize, Serialize};
use std::io;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserLoadError {
    #[error("Failed to read file: {0}")]
    ReadFileError(#[from] std::io::Error),
    #[error("Failed to parse JSON: {0}")]
    JSONDeserializeError(#[from] serde_json::Error)
}

#[derive(Error, Debug)]
pub enum UserSaveError {
    #[error("Failed to open file: {0}")]
    OpenFileError(#[from] std::io::Error),
    #[error("Failed to write JSON: {0}")]
    JSONSerializeError(#[from] serde_json::Error)
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct User {
    people: HashMap<String, Person>,
    recipes: Recipes
}

impl User {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<User, UserLoadError> {
        let file_data: String = fs::read_to_string(&path)?;
        let user = serde_json::from_str(&file_data)?;

        Ok(user)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), UserSaveError> {
        let file = fs::File::create(path)?;
        serde_json::to_writer(file, &self)?;

        Ok(())
    }

    pub fn add_person(&mut self, name: &String, person: Person) {
        self.people.insert(name.clone(), person);
    }

    pub fn remove_person(&mut self, name: &String) -> Option<Person> {
        self.people.remove(name)
    }

    pub fn find_person(&self, name: &String) -> Option<&Person> {
        self.people.get(name)
    }

    pub fn has_person(&self, name: &String) -> bool {
        self.people.contains_key(name)
    }

    pub fn get_people(&self) -> Vec<String> {
        self.people.keys().cloned().collect()
    }

    pub fn read_recipes<P: AsRef<Path>>(&mut self, folder: P) -> Result<(), io::Error> {
        self.recipes.read_recipes(folder)
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::*;
    use rstest::rstest;

    fn create_dummy_user() -> User {
        // Create person
        let mut person = Person::new("Michael", 50, 10);

        // Create user and add person
        let mut user = User::default();
        user.add_person(&person.name, person);

        user
    }

    #[rstest] // (file, expected)
    #[case("user.json", true)] // tests valid file, valid json
    #[case("", false)]         // tests invalid file, valid json
    // NOTE(mdeforge): This test assumes the JSON will never be invalid because it is being serialized from structs
    fn test_user_save<P: AsRef<Path>>(#[case] file: P, #[case] expected: bool) {
        // Create directory
        let temp_dir = TempDir::new().unwrap();
        let filename = temp_dir.join(file);

        // Create user
        let user = create_dummy_user();
        match user.save(&filename) {
            Err(UserSaveError::OpenFileError(_)) => assert!(true),
            Err(UserSaveError::JSONSerializeError(_)) => assert!(true),
            _ => assert!(expected, "Expected an UserSaveError")
        }
    }

    #[rstest]
    #[case("user.json")]
    fn test_user_save_and_load<P: AsRef<Path>>(#[case] file: P) {
        // Create directory
        let temp_dir = TempDir::new().unwrap();
        let filename = temp_dir.join(file);

        // Save user
        let save_user = create_dummy_user();
        let save_result = save_user.save(&filename);
        assert!(save_result.is_ok());

        // Load user
        let result = User::load(&filename);
        assert!(result.is_ok());

        // Get person
        let load_result = result.unwrap();
        let load_person = load_result.find_person(&String::from("Michael")).unwrap();
        assert_eq!(load_person.daily_points, 50);
    }
}
