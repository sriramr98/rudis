use crate::resp::commands::Command;

pub struct InfoCommand;

impl InfoCommand {
    pub fn new() -> Self {
        InfoCommand {}
    }
}

impl Command for InfoCommand {
    fn execute(
        &self,
        _: &std::sync::RwLock<crate::mem::MemDB<super::structs::Data>>,
    ) -> anyhow::Result<crate::resp::frame::RespFrame> {
        let response = "# Replication\r\nrole:master\r\n";
        Ok(crate::resp::frame::RespFrame::BulkString(
            response.to_string(),
        ))
    }

    fn validate(&self) -> anyhow::Result<()> {
        Ok(())
    }
}
