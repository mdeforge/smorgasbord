use crate::person::Person;
use crate::recipes::Recipes;
use serde::{Deserialize, Serialize};
use std::io;
use std::collections::HashMap;
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
    pub name: String,
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
        let dir_path = path.as_ref().parent().unwrap();
        fs::create_dir_all(dir_path)?;
        let file = fs::File::create(path)?;
        serde_json::to_writer(file, &self)?;

        Ok(())
    }

    pub fn add_person<S: AsRef<str>>(&mut self, name: S, person: Person) -> Result<(), UserSaveError> {
        self.people.insert(name.as_ref().to_string(), person);
        self.save(format!("./data/{}.user", self.name))?;

        Ok(())
    }

    pub fn remove_person(&mut self, name: &String) -> Result<(), UserSaveError> {
        self.people.remove(name);
        self.save(format!("./data/{}.user", self.name))?;

        Ok(())
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
    use std::fs::OpenOptions;
    use std::io::Write;

    fn create_dummy_user(name: &str) -> User {
        // Create person
        let person = Person::new(50, 10);

        // Create user and add person
        let mut user = User::default();
        user.add_person(name, person).unwrap();

        user
    }

    fn append_to_file<P: AsRef<Path>>(file_path: P, data: &str) -> std::io::Result<()> {
        // Open the file in append mode, creating the file if it doesn't exist
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path)?;
    
        // Write the data to the file
        file.write_all(data.as_bytes())?;
    
        Ok(())
    }

    #[rstest] // (file to save, expected)
    #[case("user.json", true)] // tests valid file, valid json
    #[case("", false)]         // tests invalid file, valid json
    // NOTE(mdeforge): This test assumes the JSON will never be invalid because it is being serialized from structs
    fn test_user_save<P: AsRef<Path>>(#[case] file: P, #[case] expected: bool) {
        // Create directory
        let temp_dir = TempDir::new().unwrap();
        let filename = temp_dir.join(file);

        // Save user
        let user = create_dummy_user("Michael");
        match user.save(&filename) {
            Err(UserSaveError::OpenFileError(_)) => assert!(true),
            Err(UserSaveError::JSONSerializeError(_)) => assert!(true),
            _ => assert!(expected, "Expected a UserSaveError")
        }
    }

    #[rstest] // (file to load, valid json, expected)
    #[case("user.json", true, true)]   // test valid file, valid json
    #[case("user.json", false, false)] // test valid file, invalid json
    #[case("", true, false)]           // test invalid file, valid json
    fn test_user_load<P: AsRef<Path>>(#[case] file: P, #[case] valid_json: bool, #[case] expected: bool) {
        // Create directory
        let temp_dir = TempDir::new().unwrap();
        let filename = temp_dir.join(file);

        // Save user
        let save_result = create_dummy_user("Michael").save("user.json");
        assert!(save_result.is_ok());

        // Mess up user data in the case we want it to fail
        if !valid_json {
            append_to_file(filename.clone(), "Foo").unwrap();
        }

        // Load user
        match User::load(filename.clone()) {
            Err(UserLoadError::ReadFileError(_)) => assert!(true),
            Err(UserLoadError::JSONDeserializeError(_)) => assert!(true),
            _ => assert!(expected, "Expected a UserLoadError")
        }
    }
}
