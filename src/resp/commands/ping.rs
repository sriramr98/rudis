use anyhow::Ok;

use crate::resp::{commands::Command, frame::RespFrame};

pub struct Ping {}

impl Command for Ping {
    fn execute(&self) -> anyhow::Result<crate::resp::frame::RespFrame> {
        Ok(RespFrame::SimpleString("PONG".to_string())) 
    }

    fn validate(&self) -> anyhow::Result<()> {
        Ok(())
    }
}