use core::fmt;
use serde::{Deserialize, Serialize};
use serde_json::to_writer;
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
pub struct Person {
    pub name: String,
    pub daily_smart_point_limit: i32,
    pub extra_points: i32
}

impl Person {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Person, PersonError> {
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
        self.daily_smart_point_limit = limit;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use assert_fs::*;

    #[rstest]
    #[case("person.json")]
    fn test_person_save_and_load<P: AsRef<Path>>(#[case] file: P) {
        let temp_dir = TempDir::new().unwrap();
        let filename = temp_dir.join(file);

        let mut save_person = Person::default();
        save_person.set_daily_smart_point_limit(50);

        let save_result = save_person.save(&filename);
        assert!(save_result.is_ok());

        let result = Person::load(&filename);
        assert!(result.is_ok());

        let load_person = result.unwrap();
        assert_eq!(load_person.daily_smart_point_limit, 50);
    }
}