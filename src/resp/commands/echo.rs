use crate::resp::commands::Command;

pub struct Echo {
    pub args: Vec<String>,
}

impl Command for Echo {
    fn execute(&self) -> anyhow::Result<crate::resp::frame::RespFrame> {
        Ok(crate::resp::frame::RespFrame::BulkString(self.args[0].clone())) //TODO: Figure out how to avoid clone as we won't need the arg after extraction.
    }

    fn validate(&self) -> anyhow::Result<()> {
        if self.args.len() != 1 {
            return Err(anyhow::Error::msg("ECHO command requires exactly one argument"));
        }
        Ok(())
    }
}