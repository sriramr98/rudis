use std::sync::RwLock;

use anyhow::{Error, Ok};

use crate::{mem::MemDB, resp::{commands::Command, frame::RespFrame}};

pub struct GetCommand {
    args: Vec<String>, 
}

impl GetCommand {
    pub fn new(args: Vec<String>) -> Self {
        return Self { args }
    }
}

impl Command for GetCommand {
    fn execute(&self, db: &RwLock<MemDB>) -> anyhow::Result<crate::resp::frame::RespFrame> {
        let key = &self.args[0];
        let d = db.read().map_err(|_| Error::msg("Unable to acquire lock"))?;

        let result = d.get(key)?;

        match result {
           Some(value)  => Ok(RespFrame::BulkString(String::from_utf8(value)?)),
           None => Ok(RespFrame::Null)
        }
    }
    
    fn validate(&self) -> anyhow::Result<()> {

        if self.args.len() != 1 {
            return Err(anyhow::anyhow!("GET command requires a key"));
        }

        if self.args[0].is_empty() {
            return Err(anyhow::anyhow!("GET command requires a key"));
        }

        Ok(())
    }
}

pub struct SetCommand {
    args: Vec<String>
}

impl SetCommand {
    pub fn new(args: Vec<String>) -> Self {
        Self{ args }
    }
}

impl Command for SetCommand {
    fn execute(&self, db: &RwLock<MemDB>) -> anyhow::Result<RespFrame> {
        let key = self.args[0].clone();
        let value = self.args[1].clone();

        let mut d = db.write().map_err(|_| Error::msg("Unable to acquire lock"))?;
        d.set(key, String::into_bytes(value));

        Ok(RespFrame::SimpleString("OK".to_string()))
    }

    fn validate(&self) -> anyhow::Result<()> {
        if self.args.len() != 2 {
            return Err(anyhow::anyhow!("SET command requires a key and value"));
        }

        Ok(())
    }
}