#![allow(unused_imports)]
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex, RwLock},
    thread,
};

mod mem;
mod resp;

use anyhow::Result;
use bytes::{BufMut, BytesMut};

use crate::{mem::MemDB, resp::commands::kv::Data};
use crate::resp::parser::parse_resp;
fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    let db = Arc::new(RwLock::new(MemDB::new()));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                handle_connection(stream, Arc::clone(&db))
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream, db: Arc<RwLock<MemDB<Data>>>) {
    thread::spawn(move || {
        loop {
            if let Err(e) = process_connection(&mut stream, &db) {
                if e.to_string() == "Connection closed" {
                    println!("connection closed");
                    break;
                }
                println!("{}", e);
            }
        }
    });
}

fn process_connection(stream: &mut TcpStream, db: &Arc<RwLock<MemDB<Data>>>) -> Result<()> {
    let mut buf = [0u8; 512];
    let bytes_read = stream.read(&mut buf)?;

    if bytes_read == 0 {
        // EOF reached - connection closed
        return Err(anyhow::anyhow!("Connection closed"));
    }

    let command = std::str::from_utf8(&buf)?;

    match parse_resp(command) {
        Ok(request) => {
            request.validate()?;
            let resp = request.execute(db.as_ref())?;
            stream.write_all(&resp.encode())?;
        }
        Err(e) => {
            // For errors, we might want to return an error to the client
            // and decide if we should close the connection.
            // For now, just print and continue.
            println!("Error parsing command: {}", e);
            // Simple error response
            stream.write_all(b"-ERR invalid command\r\n")?;
        }
    }

    Ok(())
}
