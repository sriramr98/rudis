use std::pin::Pin;

use anyhow::{Error, Ok, Result};

use crate::resp::commands::{Command, GetCommand, Ping, echo::Echo, kv::SetCommand};

// RESP parser that returns appropriate Command implementations
pub fn parse_resp(input: &str) -> Result<Box<dyn Command>> {
    let input = input.trim_end();
    // Check if input is empty
    if input.is_empty() {
        return Err(Error::msg("Empty input"));
    }

    if !input.starts_with("*") {
        return Err(Error::msg("Invalid RESP array"));
    }

    let lines = input.lines().collect::<Vec<&str>>();
    if lines.is_empty() {
        return Err(Error::msg("Invalid input"));
    }

    let line_count = lines.len();

    if line_count < 3 {
        // We need a minimum of three lines
        // First line for the array size
        // second and third line for the command as bulk string
        return Err(Error::msg("Invalid input"))
    }

    let param_count: usize = lines[0][1..].parse().map_err(|_| Error::msg("Invalid command count"))?;
    let mut args: Vec<String> = Vec::with_capacity(param_count - 1); // -1 because the count also includes the command

    // First two lines are used to parse the command type
    let _: usize = lines[1][1..].parse().map_err(|_| Error::msg("Invalid request format"))?; // We only parse for the protocol agreement
    let command = lines[2].to_string();
    let mut idx = 3; // ofsetting idx to start parsing args

    // last line needs to be ignore
 
    while idx < line_count -1 {
        let line = lines[idx];
        if !line.starts_with("$") {
            return Err(Error::msg("Invalid bulk string"));
        }
        let _: usize = line[1..].parse().map_err(|_| Error::msg("Invalid bulk string length"))?; // We only parse to agree on the protocol and throw error. We actually don't need the size for our implementation
        idx += 1;

        if idx >= line_count {
            // We have a malformatted request since we have size but no content in bulk string
            return Err(Error::msg("Malformatted request"))
        }

        let arg = lines[idx].to_string();
        args.push(arg);
        idx += 1;
    }

    match command.to_lowercase().as_str() {
        "ping" => Ok(Box::new(Ping{})),
        "echo" => Ok(Box::new(Echo{ args })),
        "get" => Ok(Box::new(GetCommand::new(args))),
        "set" => Ok(Box::new(SetCommand::new(args))),
        _ => Err(Error::msg("Unknown command"))
    }
}