use std::sync::RwLock;

use anyhow::Result;

use crate::{
    config::Config,
    mem::MemDB,
    resp::{commands::structs::Data, frame::RespFrame},
};

pub trait Command: Send {
    fn execute(&self, db: &RwLock<MemDB<Data>>, config: &Config) -> Result<RespFrame>;
    fn validate(&self) -> Result<()>;
}
