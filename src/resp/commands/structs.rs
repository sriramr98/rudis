use std::time::Instant;

type BinaryString = Vec<u8>;

#[derive(Clone)]
pub enum Value {
    String(BinaryString),
    List(Vec<String>),
}

impl Value {
    pub fn to_vec(self: &Self) -> Vec<u8> {
        match self {
            Value::String(v)  => v.clone(),
            Value::List(items) => todo!(),
        } 
    }
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