use std::{collections::HashMap, sync::{Arc, RwLock}, time::Instant};

use anyhow::{Error, Ok, Result};
    
type BinaryString = Vec<u8>;

pub enum Value {
    String(BinaryString)
}


pub struct MemDB {
    store: HashMap<String, Value>
}

impl MemDB {
    pub fn new() -> Self {
        MemDB {
            store: HashMap::new()
        }
    }

    pub fn get(&self, key: &str) -> Result<Option<Vec<u8>>> {
        let cloned: Option<&Value> = self.store.get(key);

        match cloned {
            Some(Value::String(v)) => {
                Ok(Some(v.clone()))
            }
            None => Ok(None)
        }

    }

    pub fn set(&mut self, key: String, value: Vec<u8>) {
        self.store.insert(key, Value::String(value));
    }
}