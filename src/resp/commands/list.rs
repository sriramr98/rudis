use anyhow::Ok;

use crate::resp::{
    commands::{
        Command,
        structs::{Data, Value},
    },
    frame::RespFrame,
};

// RPUSH implementaion
pub struct ListPushCommand {
    args: Vec<String>,
    reverse: bool,
}

impl ListPushCommand {
    pub fn new(args: Vec<String>, reverse: bool) -> Self {
        Self { args, reverse }
    }
}

impl Command for ListPushCommand {
    fn execute(
        &self,
        db: &std::sync::RwLock<crate::mem::MemDB<super::structs::Data>>,
        _config: &crate::config::Config,
    ) -> anyhow::Result<crate::resp::frame::RespFrame> {
        let key = &self.args[0];
        let mut values: Vec<String> = self.args[1..].iter().map(|s| s.clone()).collect();

        if self.reverse {
            values.reverse();
        }

        let mut db_write = db.write().unwrap();
        let data = db_write.get(key)?;

        let list_data = match data {
            Some(d) => match &d.value {
                super::structs::Value::List(items) => {
                    let mut new_items = items.clone();
                    if self.reverse {
                        new_items.splice(0..0, values);
                    } else {
                        new_items.extend(values);
                    }
                    new_items
                }
                _ => {
                    return Err(anyhow::anyhow!(
                        "WRONGTYPE Operation against a key holding the wrong kind of value"
                    ));
                }
            },
            None => values,
        };

        db_write.set(
            key.to_string(),
            Data {
                value: super::structs::Value::List(list_data.clone()),
                expires_at: None,
            },
        );

        let count = list_data.len() as i64;

        Ok(crate::resp::frame::RespFrame::Integer(count))
    }

    fn validate(&self) -> anyhow::Result<()> {
        if self.args.len() < 2 {
            return Err(anyhow::anyhow!(
                "ERR wrong number of arguments for 'rpush' command"
            ));
        }
        Ok(())
    }
}

// LRANGE implementation
pub struct ListGetCommand {
    args: Vec<String>,
}

impl ListGetCommand {
    pub fn new(args: Vec<String>) -> Self {
        Self { args }
    }
}

impl Command for ListGetCommand {
    fn execute(
        &self,
        db: &std::sync::RwLock<crate::mem::MemDB<Data>>,
        _config: &crate::config::Config,
    ) -> anyhow::Result<crate::resp::frame::RespFrame> {
        let key = self.args[0].clone();

        let mut start: isize = self.args[1].parse().unwrap_or(0);
        let mut end: isize = self.args[2].parse().unwrap_or(0);

        let db_read = db.read().unwrap();
        match db_read.get(&key)? {
            Some(data) => match &data.value {
                super::structs::Value::List(items) => {
                    let total_items = items.len() as isize;

                    if start < 0 {
                        start = total_items + start;
                    }

                    if end < 0 {
                        end = total_items + end;
                    }

                    if start > end {
                        return Ok(crate::resp::frame::RespFrame::EmptyArray);
                    }

                    if start >= total_items {
                        return Ok(crate::resp::frame::RespFrame::EmptyArray);
                    }

                    let slice_end: isize = if end + 1 > total_items {
                        total_items
                    } else {
                        end + 1
                    };

                    let start_usize = start.max(0) as usize;
                    let end_usize = slice_end.max(0) as usize;

                    println!("LRANGE slicing from {} to {}", start_usize, end_usize);

                    let sliced_items = &items[start_usize..end_usize];
                    let resp_frames: Vec<crate::resp::frame::RespFrame> = sliced_items
                        .iter()
                        .map(|item| crate::resp::frame::RespFrame::BulkString(item.clone()))
                        .collect();
                    return Ok(crate::resp::frame::RespFrame::Array(resp_frames));
                }
                _ => {
                    return Err(anyhow::anyhow!(
                        "WRONGTYPE Operation against a key holding the wrong kind of value"
                    ));
                }
            },
            None => return Ok(crate::resp::frame::RespFrame::EmptyArray),
        }
    }

    fn validate(&self) -> anyhow::Result<()> {
        if self.args.len() != 3 {
            return Err(anyhow::anyhow!(
                "ERR wrong number of arguments for 'lrange' command"
            ));
        }

        let _start: isize = self.args[1]
            .parse()
            .map_err(|_| anyhow::anyhow!("ERR value is not an integer or out of range"))?;
        let _end: isize = self.args[2]
            .parse()
            .map_err(|_| anyhow::anyhow!("ERR value is not an integer or out of range"))?;

        Ok(())
    }
}

pub struct ListLengthCommand {
    args: Vec<String>,
}

impl ListLengthCommand {
    pub fn new(args: Vec<String>) -> Self {
        Self { args }
    }
}

impl Command for ListLengthCommand {
    fn execute(
        &self,
        db: &std::sync::RwLock<crate::mem::MemDB<super::structs::Data>>,
        _config: &crate::config::Config,
    ) -> anyhow::Result<crate::resp::frame::RespFrame> {
        let key = &self.args[0];

        let db_read = db.read().unwrap();
        match db_read.get(key)? {
            Some(d) => match &d.value {
                super::structs::Value::List(items) => {
                    let count = items.len() as i64;
                    Ok(crate::resp::frame::RespFrame::Integer(count))
                }
                _ => Err(anyhow::anyhow!(
                    "WRONGTYPE Operation against a key holding the wrong kind of value"
                )),
            },
            None => Ok(crate::resp::frame::RespFrame::Integer(0)),
        }
    }

    fn validate(&self) -> anyhow::Result<()> {
        if self.args.len() != 1 {
            return Err(anyhow::anyhow!(
                "ERR wrong number of arguments for 'llen' command"
            ));
        }
        Ok(())
    }
}

pub struct ListPopCommand {
    args: Vec<String>,
}

impl ListPopCommand {
    pub fn new(args: Vec<String>) -> Self {
        Self { args }
    }
}

impl Command for ListPopCommand {
    fn execute(
        &self,
        db: &std::sync::RwLock<crate::mem::MemDB<super::structs::Data>>,
        _config: &crate::config::Config,
    ) -> anyhow::Result<crate::resp::frame::RespFrame> {
        let key = self.args[0].clone();

        // by default, we only need to remove the first element.
        let remove_start = 0;
        let mut remove_end = 1;

        if self.args.len() == 2 {
            // user provides the count of elements to remove
            let count: usize = self.args[1]
                .parse()
                .map_err(|_| anyhow::anyhow!("ERR value is not an integer or out of range"))?;
            if count == 0 {
                return Ok(RespFrame::NullBulkString);
            }

            remove_end = count;
        }

        let mut db_write = db.write().map_err(|e| anyhow::anyhow!(e.to_string()))?;

        let (mut new_list, expires_at) = {
            let Some(data) = db_write.get(&key)? else {
                return Ok(RespFrame::NullBulkString);
            };

            let Value::List(items) = &data.value else {
                return Err(anyhow::anyhow!(
                    "WRONGTYPE Operation against a key holding the wrong kind of value"
                ));
            };

            (items.clone(), data.expires_at)
        };

        if remove_end > new_list.len() {
            remove_end = new_list.len();
        }

        let remove_result = new_list.drain(remove_start..remove_end);
        let popped_elements: Vec<RespFrame> =
            remove_result.map(|s| RespFrame::BulkString(s)).collect();

        db_write.set(
            key,
            Data {
                value: Value::List(new_list),
                expires_at,
            },
        );

        if popped_elements.len() == 1 {
            Ok(popped_elements[0].clone())
        } else {
            Ok(RespFrame::Array(popped_elements))
        }
    }

    fn validate(&self) -> anyhow::Result<()> {
        if self.args.is_empty() || self.args.len() > 2 {
            return Err(anyhow::anyhow!(
                "ERR wrong number of arguments for 'lpop' command"
            ));
        }
        Ok(())
    }
}
