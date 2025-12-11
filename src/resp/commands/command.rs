use anyhow::Result;

use crate::resp::frame::RespFrame;

pub trait Command {
    fn execute(&self) -> Result<RespFrame>;
    fn validate(&self) -> Result<()>;
}