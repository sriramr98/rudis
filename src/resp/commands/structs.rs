use std::{collections::VecDeque, time::Instant};

type BinaryString = Vec<u8>;

#[derive(Clone)]
pub enum Value {
    String(BinaryString),
    List(Vec<String>),
}

// Data wraps over Value with extra metadata
pub struct Data {
    pub value: Value,
    pub expires_at: Option<Instant>
}

impl Data {
    pub fn expired(&self) -> bool {
        match self.expires_at {
            Some(expiry) => Instant::now() >= expiry,
            None => false,
        }
    }
}