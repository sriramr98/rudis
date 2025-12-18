use std::sync::RwLock;

use anyhow::Result;

use crate::{mem::MemDB, resp::frame::RespFrame};

pub trait Command {
    fn execute(&self, db: &RwLock<MemDB>) -> Result<RespFrame>;
    fn validate(&self) -> Result<()>;
}