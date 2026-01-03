use std::sync::RwLock;

use anyhow::Ok;

use crate::{
    mem::MemDB,
    resp::{
        commands::{Command, structs::Data},
        frame::RespFrame,
    },
};

pub struct Ping {}

impl Command for Ping {
    fn execute(
        &self,
        _: &RwLock<MemDB<Data>>,
        _config: &crate::config::Config,
    ) -> anyhow::Result<crate::resp::frame::RespFrame> {
        Ok(RespFrame::SimpleString("PONG".to_string()))
    }

    fn validate(&self) -> anyhow::Result<()> {
        Ok(())
    }
}
