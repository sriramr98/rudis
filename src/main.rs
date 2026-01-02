#![allow(unused_imports)]
mod connection;
mod mem;
mod resp;

use std::sync::{Arc, Mutex, RwLock};

use anyhow::{Ok, Result};
use bytes::{BufMut, BytesMut};
use clap::Parser;
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

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 6379)]
    port: u32,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Logs from your program will appear here!");

    let args = Args::parse();
    let listener_url = format!("127.0.0.1:{}", args.port);

    println!("Listening on {}", listener_url);

    let listener = TcpListener::bind(listener_url).await?;
    let db = Arc::new(RwLock::new(MemDB::<Data>::new()));

    loop {
        let (stream, addr) = listener.accept().await?;
        println!("New Connection from {}:{}", addr.ip(), addr.port());

        let connection = Connection::new(stream, addr);

        let db_clone = Arc::clone(&db);
        tokio::spawn(async move {
            if let Err(e) = handle_connection(connection, db_clone).await {
                eprintln!("Error handling connection: {}", e);
            }
        });
    }
}

async fn handle_connection(mut connection: Connection, db: Arc<RwLock<MemDB<Data>>>) -> Result<()> {
    loop {
        match connection.parse().await {
            Err(err) => {
                if err.to_string().starts_with("Connection closed") {
                    println!("{}", err);
                    return Ok(());
                }
                println!("Error parsing command: {}", err);
                let _ = connection.write(RespFrame::Error(err.to_string())).await; //TODO: How to handle errors here??
            }
            std::result::Result::Ok(command) => {
                command.validate()?;
                let result = command.execute(db.as_ref());
                connection.write_result(result).await?;
            }
        }
    }
}
