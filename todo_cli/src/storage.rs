use crate::todo::Todo;
use std::fs;
use std::io;
use std::path::Path;

const STORAGE_FILE: &str = "todos.json";

pub struct Storage {
    file_path: String,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            file_path: STORAGE_FILE.to_string(),
        }
    }

    pub fn load(&self) -> Result<Vec<Todo>, io::Error> {
        if !Path::new(&self.file_path).exists() {
            return Ok(Vec::new());
        }

        let contents = fs::read_to_string(&self.file_path)?;
        let todos: Vec<Todo> = serde_json::from_str(&contents)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(todos)
    }

    pub fn save(&self, todos: &[Todo]) -> Result<(), io::Error> {
        let json = serde_json::to_string_pretty(todos)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        fs::write(&self.file_path, json)?;
        Ok(())
    }
}