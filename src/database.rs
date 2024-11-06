/// This module contains a thread-safe in-memory datastore for the webserver.
use std::collections::HashMap;
use std::sync::Mutex;

const DB_PATH: &str = "db.txt";

pub struct Database {
    data: Mutex<HashMap<String, String>>,
    path: String,
}

impl Database {
    pub fn new() -> Database {
        // Read the database from disk

        let db = Database {
            data: Mutex::new(HashMap::new()),
            path: DB_PATH.to_string(),
        };
        db.from_file();
        db
    }

    // Read the database from disk
    fn from_file(&self) {
        if !std::path::Path::new(&self.path).exists() {
            std::fs::File::create(&self.path).expect("Failed to create database file!");
        }

        // Read the file
        let data = std::fs::read_to_string(&self.path).expect("Failed to read database file!");

        // Parse the data
        let mut map = self.data.lock().unwrap();
        for line in data.lines() {
            let mut parts = line.split('=');
            let key = parts.next().expect("Failed to parse key!");
            let value = parts.next().expect("Failed to parse value!");
            map.insert(key.to_string(), value.to_string());
        }
    }

    // Write the database to disk
    fn to_file(&self) {
        if !std::path::Path::new(&self.path).exists() {
            std::fs::File::create(&self.path).expect("Failed to create database file!");
        }

        // Encode the data
        let data = self
            .data
            .lock()
            .unwrap()
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<String>>()
            .join("\n");

        // Write to file
        std::fs::write(&self.path, data).expect("Failed to write to database file!");
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.data.lock().unwrap().get(key).cloned()
    }

    pub fn set(&self, key: String, value: String) {
        self.data.lock().unwrap().insert(key, value);
        self.to_file();
    }

    pub fn delete(&self, key: &str) -> Option<String> {
        let result = self.data.lock().unwrap().remove(key);
        self.to_file();
        result
    }
}
