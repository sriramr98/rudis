#![allow(unused_imports)]
mod config;
mod connection;
mod mem;
mod resp;

use std::sync::{Arc, Mutex, RwLock};

use anyhow::{Ok, Result};
use bytes::{BufMut, BytesMut};
use clap::Parser;
use tokio::net::{TcpListener, TcpStream};

use crate::{
    config::{Config, Role},
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

    #[arg(short, long)]
    replicaof: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Logs from your program will appear here!");

    let args = Args::parse();
    let config = parse_config(&args);

    let listener_url = format!("127.0.0.1:{}", args.port);

    println!("Listening on {}", listener_url);

    let listener = TcpListener::bind(listener_url).await?;
    let db = Arc::new(RwLock::new(MemDB::<Data>::new()));

    loop {
        let (stream, addr) = listener.accept().await?;
        println!("New Connection from {}:{}", addr.ip(), addr.port());

        let connection = Connection::new(stream, addr);

        let db_clone = Arc::clone(&db);
        let config_clone = config.clone();
        config_clone.increment_connections();
        tokio::spawn(async move {
            if let Err(e) = handle_connection(connection, db_clone, config_clone.clone()).await {
                eprintln!("Error handling connection: {}", e);
            }
            config_clone.decrement_connections();
        });
    }
}

fn parse_config(args: &Args) -> Config {
    let role: Role;
    let mut master_host: Option<String> = None;
    let mut master_port: Option<u32> = None;

    if let Some(replicaof_str) = &args.replicaof {
        role = Role::Replica;

        // Parse "host port" format
        let parts: Vec<&str> = replicaof_str.split_whitespace().collect();
        if parts.len() == 2 {
            master_host = Some(parts[0].to_string());
            master_port = parts[1].parse().ok();
        }
    } else {
        role = Role::Master
    }

    Config::new(role, args.port, master_host, master_port)
}

async fn handle_connection(
    mut connection: Connection,
    db: Arc<RwLock<MemDB<Data>>>,
    config: Config,
) -> Result<()> {
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
                let result = command.execute(db.as_ref(), &config);
                connection.write_result(result).await?;
            }
        }
    }
}
