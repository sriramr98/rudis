use std::sync::RwLock;

use anyhow::Result;

use crate::{mem::MemDB, resp::{commands::structs::Data, frame::RespFrame}};

pub trait Command {
    fn execute(&self, db: &RwLock<MemDB<Data>>) -> Result<RespFrame>;
    fn validate(&self) -> Result<()>;
}