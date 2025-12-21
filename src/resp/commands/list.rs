use anyhow::Ok;

use crate::resp::commands::{Command, structs::Data};

// RPUSH implementaion
pub struct ListPushCommand {
    args: Vec<String>
}

impl ListPushCommand {
    pub fn new(args: Vec<String>) -> Self {
        Self { args }
    }
}

impl Command for ListPushCommand {
    fn execute(&self, db: &std::sync::RwLock<crate::mem::MemDB<super::structs::Data>>) -> anyhow::Result<crate::resp::frame::RespFrame> {
        self.validate()?;

        let key = &self.args[0];
        let values: Vec<String> = self.args[1..]
            .iter()
            .map(|s| s.clone())
            .collect();

        let mut db_write = db.write().unwrap();
        let data = db_write.get(key)?;

        let list_data = match data {
            Some(d) => match &d.value {
                super::structs::Value::List(items) => {
                    let mut new_items = items.clone();
                    new_items.extend(values);
                    new_items
                },
                _ => {
                    return Err(anyhow::anyhow!("WRONGTYPE Operation against a key holding the wrong kind of value"));
                }
            },
            None => values,
        };

        db_write.set(key.to_string(), Data{
            value: super::structs::Value::List(list_data.clone()),
            expires_at: None
        });

        let count = list_data.len() as i64;

        Ok(crate::resp::frame::RespFrame::Integer(count))

    }

    fn validate(&self) -> anyhow::Result<()> {
        if self.args.len() < 2 {
            return Err(anyhow::anyhow!("ERR wrong number of arguments for 'rpush' command"));
        }
        Ok(())
    }
}

// LRANGE implementation
pub struct ListGetCommand {
    args: Vec<String>
}

impl ListGetCommand {
    pub fn new(args: Vec<String>) -> Self {
        Self { args }
    }
}

impl Command for ListGetCommand {
    fn execute(&self, db: &std::sync::RwLock<crate::mem::MemDB<Data>>) -> anyhow::Result<crate::resp::frame::RespFrame> {
        let key = self.args[0].clone();

        let start: usize = self.args[1].parse().unwrap_or(0);
        let end: usize = self.args[2].parse().unwrap_or(0);

        if start > end {
            return Ok(crate::resp::frame::RespFrame::EmptyArray);
        }

        let db_read = db.read().unwrap();
        match db_read.get(&key)? {
            Some(data) => {
                match &data.value {
                    super::structs::Value::List(items) => {
                        let total_items = items.len();

                        if start >= total_items {
                            return Ok(crate::resp::frame::RespFrame::EmptyArray);
                        }

                        let slice_end = if end + 1 > items.len() { items.len() } else { end + 1 };
                        let sliced_items = &items[start..slice_end];
                        let resp_frames: Vec<crate::resp::frame::RespFrame> = sliced_items.iter()
                            .map(|item| crate::resp::frame::RespFrame::BulkString(item.clone()))
                            .collect();
                        return Ok(crate::resp::frame::RespFrame::Array(resp_frames));
                    },
                    _ => {
                        return Err(anyhow::anyhow!("WRONGTYPE Operation against a key holding the wrong kind of value"));
                    }
                }
            },
            None => {
                return Ok(crate::resp::frame::RespFrame::EmptyArray)
            }
        }


    }

    fn validate(&self) -> anyhow::Result<()> {
        if self.args.len() != 3 {
            return Err(anyhow::anyhow!("ERR wrong number of arguments for 'lrange' command"));
        }

        let _start: usize = self.args[1].parse().map_err(|_| anyhow::anyhow!("ERR value is not an integer or out of range"))?;
        let _end: usize = self.args[2].parse().map_err(|_| anyhow::anyhow!("ERR value is not an integer or out of range"))?;

        Ok(())
    }
}