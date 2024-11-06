/// This module contains a thread-safe in-memory datastore for the webserver.
use std::collections::HashMap;
use std::sync::Mutex;

pub struct Database {
    data: Mutex<HashMap<String, String>>,
}

impl Database {
    pub fn new() -> Database {
        Database {
            data: Mutex::new(HashMap::new()),
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.data.lock().unwrap().get(key).cloned()
    }

    pub fn set(&self, key: String, value: String) {
        self.data.lock().unwrap().insert(key, value);
    }

    pub fn delete(&self, key: &str) -> Option<String> {
        self.data.lock().unwrap().remove(key)
    }
}
