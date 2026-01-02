use std::net::SocketAddr;

use anyhow::{Result, anyhow};
use bytes::BytesMut;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter},
    net::TcpStream,
};

use crate::resp::{commands::Command, frame::RespFrame, parser::parse_resp};

pub struct Connection {
    stream: BufWriter<TcpStream>,
    addr: SocketAddr,
}

impl Connection {
    pub fn new(stream: TcpStream, addr: SocketAddr) -> Connection {
        Connection {
            stream: BufWriter::new(stream),
            addr,
        }
    }

    pub async fn parse(&mut self) -> Result<Box<dyn Command>> {
        //TODO: For MVP, 4KB is more than enough, but we should let users configure it based on requirement
        let mut buf = BytesMut::with_capacity(4 * 1024);
        let bytes_read = self.stream.read_buf(&mut buf).await?;

        if bytes_read == 0 {
            return Err(anyhow!(
                "Connection closed by {}:{}",
                self.addr.ip(),
                self.addr.port()
            ));
        }

        let data = std::str::from_utf8(&buf)?;

        let command = parse_resp(data)?;

        Ok(command)
    }

    pub async fn write(&mut self, frame: RespFrame) -> Result<()> {
        self.stream.write_all(&frame.encode()).await?;
        self.stream.flush().await?;
        Ok(())
    }

    pub async fn write_result(
        &mut self,
        result: std::result::Result<RespFrame, anyhow::Error>,
    ) -> Result<()> {
        match result {
            Ok(frame) => self.write(frame).await,
            Err(err) => self.write(RespFrame::Error(err.to_string())).await,
        }
    }
}
