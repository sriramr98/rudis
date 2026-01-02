#![allow(unused_imports)]
mod connection;
mod mem;
mod resp;

use std::sync::{Arc, Mutex, RwLock};

use anyhow::{Ok, Result};
use bytes::{BufMut, BytesMut};
use tokio::net::{TcpListener, TcpStream};

use crate::{
    connection::Connection,
    resp::{
        commands::{Command, list, structs::Value},
        parser::parse_resp,
    },
};
use crate::{
    mem::MemDB,
    resp::{commands::structs::Data, frame::RespFrame},
};

#[tokio::main]
async fn main() -> Result<()> {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    let db = Arc::new(RwLock::new(MemDB::<Data>::new()));

    loop {
        let (stream, addr) = listener.accept().await?;
        println!("New Connection from {}:{}", addr.ip(), addr.port());
        let db_clone = Arc::clone(&db);
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, db_clone).await {
                eprintln!("Error handling connection: {}", e);
            }
        });
    }
}

async fn handle_connection(stream: TcpStream, db: Arc<RwLock<MemDB<Data>>>) -> Result<()> {
    let mut connection = Connection::new(stream);
    match connection.parse().await {
        Err(err) => {
            println!("Error parsing command: {}", err);
            let _ = connection.write(RespFrame::Error(err.to_string())).await; //TODO: How to handle errors here??
            Ok(())
        }
        std::result::Result::Ok(command) => {
            command.validate()?;
            let result = command.execute(db.as_ref());
            connection.write_result(result).await?;
            Ok(())
        }
    }
}
