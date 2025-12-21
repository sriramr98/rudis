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