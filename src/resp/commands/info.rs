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
        config: &crate::config::Config,
    ) -> anyhow::Result<crate::resp::frame::RespFrame> {
        let response = format!(
            "#Clients\r\nconnected_clients:{}\r\n# Replication\r\nrole:{}\r\n",
            config.connected_clients(),
            config.role(),
        );
        Ok(crate::resp::frame::RespFrame::BulkString(response))
    }

    fn validate(&self) -> anyhow::Result<()> {
        Ok(())
    }
}
