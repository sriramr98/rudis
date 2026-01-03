use std::sync::RwLock;

use crate::{
    mem::MemDB,
    resp::commands::{Command, structs::Data},
};

pub struct Echo {
    pub args: Vec<String>,
}

impl Command for Echo {
    fn execute(
        &self,
        _: &RwLock<MemDB<Data>>,
        _config: &crate::config::Config,
    ) -> anyhow::Result<crate::resp::frame::RespFrame> {
        Ok(crate::resp::frame::RespFrame::BulkString(
            self.args[0].clone(),
        )) //TODO: Figure out how to avoid clone as we won't need the arg after extraction.
    }

    fn validate(&self) -> anyhow::Result<()> {
        if self.args.len() != 1 {
            return Err(anyhow::Error::msg(
                "ECHO command requires exactly one argument",
            ));
        }
        Ok(())
    }
}
