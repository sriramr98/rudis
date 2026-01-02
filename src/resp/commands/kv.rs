use std::{
    sync::RwLock,
    time::{Duration, Instant},
};

use anyhow::{Error, Ok};

use crate::{
    mem::MemDB,
    resp::{commands::{Command, structs::{Data, Value}}, frame::RespFrame},
};

pub struct GetCommand {
    args: Vec<String>,
}

impl GetCommand {
    pub fn new(args: Vec<String>) -> Self {
        return Self { args };
    }
}

impl Command for GetCommand {
    fn execute(&self, db: &RwLock<MemDB<Data>>) -> anyhow::Result<crate::resp::frame::RespFrame> {
        let key = &self.args[0];
        let d = db
            .read()
            .map_err(|_| Error::msg("Unable to acquire lock"))?;

        let result = d.get(key)?;

        match result {
            Some(value) => {
                if value.expired() {
                    return Ok(RespFrame::NullBulkString);
                }

                match &value.value {
                    Value::String(data) => Ok(RespFrame::BulkString(String::from_utf8(data.clone())?)),
                    Value::List(_) => {
                        return Err(anyhow::anyhow!("GET command does not support List values"));
                    },
                }
            }
            None => Ok(RespFrame::Null),
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
    args: Vec<String>,
}

impl SetCommand {
    pub fn new(args: Vec<String>) -> Self {
        Self { args }
    }
}

impl Command for SetCommand {
    fn execute(&self, db: &RwLock<MemDB<Data>>) -> anyhow::Result<RespFrame> {
        let key = self.args[0].clone();
        let value = self.args[1].clone();

        let mut expires_at: Option<Instant> = None;
        if self.args.len() == 4 {
            // expiry is set
            let expiry_type = self.args[2].clone().to_lowercase();
            let expiry_val: u64 = self.args[3].clone().parse()?;

            let timestamp = Instant::now();

            expires_at = match expiry_type.as_str() {
                "px" => timestamp.checked_add(Duration::from_millis(expiry_val)),
                "ex" => timestamp.checked_add(Duration::from_secs(expiry_val)),
                _ => None,
            };
        }

        let mut d = db
            .write()
            .map_err(|_| Error::msg("Unable to acquire lock"))?;

        let data = Data {
            value: Value::String(String::into_bytes(value.clone())),
            expires_at,
        };
        d.set(key, data);

        Ok(RespFrame::SimpleString("OK".to_string()))
    }

    fn validate(&self) -> anyhow::Result<()> {
        if self.args.len() < 2 {
            return Err(anyhow::anyhow!("SET command requires a key and value"));
        }

        if self.args.len() == 2 {
            return Ok(()); // Simple SET key value
        }

        if self.args.len() != 4 {
            return Err(anyhow::anyhow!("Malformatted SET, expiry missing"));
        }

        //TODO: Validate expiry type to be PX or EX only..

        Ok(())
    }
}
