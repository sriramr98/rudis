use std::{collections::HashMap, sync::{Arc, RwLock}, time::Instant};

use anyhow::{Error, Ok, Result};
    

pub struct MemDB<T> {
    store: HashMap<String, T>
}

impl<T> MemDB<T> {
    pub fn new() -> Self {
        MemDB {
            store: HashMap::new()
        }
    }

    pub fn get(&self, key: &str) -> Result<Option<&T>> {
        let cloned: Option<&T> = self.store.get(key);

        match cloned {
            Some(vs) => {
                Ok(Some(vs))
            }
            None => Ok(None)
        }

    }

    pub fn set(&mut self, key: String, data: T) {
        self.store.insert(key, data);
    }
}