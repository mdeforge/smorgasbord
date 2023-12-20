use core::fmt;
use serde::{Deserialize, Serialize};
use serde_json::to_writer;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::collections::HashMap;
use crate::person::Person;

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

// impl From<std::io::Error> for UserError {
//     fn from(error: std::io::Error) -> Self {
//         UserError::WriteError(error)
//     }
// }

impl From<serde_json::Error> for UserError {
    fn from(error: serde_json::Error) -> Self {
        UserError::ParseError(error)
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct User {
    people: HashMap<String, Person>
}

impl User {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<User, UserError> {
        let file_data: String = fs::read_to_string(&path)?;
        let user = serde_json::from_str(&file_data)?;

        Ok(user)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn Error>> {
        let file = fs::File::create(path)?;
        serde_json::to_writer(file, &self)?;

        Ok(())
    }

    pub fn add_person(&mut self, name: String, person: Person) {
        self.people.insert(name, person);
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use assert_fs::*;

    #[rstest]
    #[case("user.json")]
    fn test_user_save_and_load<P: AsRef<Path>>(#[case] file: P) {
        // Create directory
        let temp_dir = TempDir::new().unwrap();
        let filename = temp_dir.join(file);

        // Create person
        let mut person = Person::default();
        person.set_daily_smart_point_limit(50);

        // Create user and add person
        let mut save_user = User::default();
        save_user.add_person(String::from("Michael"), person);

        // Save user
        let save_result = save_user.save(&filename);
        assert!(save_result.is_ok());

        // Load user
        let result = User::load(&filename);
        assert!(result.is_ok());

        // Get person
        let load_result = result.unwrap();
        let load_person = load_result.find_person(&String::from("Michael")).unwrap();
        assert_eq!(load_person.daily_smart_point_limit, 50);
    }
}