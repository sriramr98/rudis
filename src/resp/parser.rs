use anyhow::{Error, Ok, Result};

use crate::resp::commands::{Command, Ping};

pub fn parse_resp(input: &str) -> Result<Box<dyn Command>> {
    Ok(Box::new(Ping{}))
}