use crate::user::User;
use crate::recipes::RecipeBox;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AccountLoadError {
    #[error("Failed to read file: {0}")]
    ReadFileError(#[from] std::io::Error),
    #[error("Failed to parse JSON: {0}")]
    JSONDeserializeError(#[from] serde_json::Error)
}

#[derive(Error, Debug)]
pub enum AccountSaveError {
    #[error("Failed to open file: {0}")]
    OpenFileError(#[from] std::io::Error),
    #[error("Failed to write JSON: {0}")]
    JSONSerializeError(#[from] serde_json::Error)
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Account {
    pub name: String,
    recipe_box_path: String,
    people: HashMap<String, User>,
    recipe_box: RecipeBox,
}

impl Account {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Account, AccountLoadError> {
        let file_data: String = fs::read_to_string(&path)?;
        let user = serde_json::from_str(&file_data)?;

        Ok(user)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), AccountSaveError> {
        let dir_path = path.as_ref().parent().unwrap();
        fs::create_dir_all(dir_path)?;
        let file = fs::File::create(path)?;
        serde_json::to_writer(file, &self)?;

        Ok(())
    }

    pub fn add_user<S: AsRef<str>>(&mut self, name: S, user: User) -> Result<(), AccountSaveError> {
        self.people.insert(name.as_ref().to_string(), user);
        self.save(format!("./data/{}.user", self.name))?;

        Ok(())
    }

    pub fn remove_user(&mut self, name: &String) -> Result<(), AccountSaveError> {
        self.people.remove(name);
        self.save(format!("./data/{}.user", self.name))?;

        Ok(())
    }

    pub fn find_user(&mut self, name: String) -> Option<&mut User> {
        self.people.get_mut(&name)
    }

    pub fn has_person(&self, name: &String) -> bool {
        self.people.contains_key(name)
    }

    pub fn get_people(&self) -> Vec<String> {
        self.people.keys().cloned().collect()
    }

    pub fn recipe_box(&mut self) -> &mut RecipeBox {
        &mut self.recipe_box
    }

    pub fn recipe_path(&self) -> String {
        self.recipe_box_path.clone()
    }

    pub fn set_recipe_path(&mut self, path: String) {
        self.recipe_box_path = path;
    }

    // NOTE(mdeforge): These should not be a part of user
    // pub fn read_recipes<P: AsRef<Path>>(&mut self, folder: P) -> Result<(), io::Error> {
    //     self.recipes.read_recipes(folder)
    // }

    // pub fn get_recipe_names(&mut self) -> Vec<String> {
    //     self.recipes.get_recipe_names()
    // }

}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::*;
    use rstest::rstest;
    use std::fs::OpenOptions;
    use std::io::Write;

    fn create_dummy_user(name: &str) -> Account {
        // Create user
        let user = User::new(50, 10);

        // Create user and add person
        let mut account = Account::default();
        account.add_user(name, user).unwrap();

        account
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
            Err(AccountSaveError::OpenFileError(_)) => assert!(true),
            Err(AccountSaveError::JSONSerializeError(_)) => assert!(true),
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
        match Account::load(filename.clone()) {
            Err(AccountLoadError::ReadFileError(_)) => assert!(true),
            Err(AccountLoadError::JSONDeserializeError(_)) => assert!(true),
            _ => assert!(expected, "Expected a UserLoadError")
        }
    }
}
