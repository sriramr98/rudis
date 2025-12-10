#![allow(unused_imports)]
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use anyhow::Result;

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                if let Err(e) = handle_connection(stream) {
                    println!("{}", e)
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let mut buf = [0u8; 512];
    let bytes_read = stream.read(&mut buf)?;

    let command =  str::from_utf8(&buf[0..bytes_read])?;

    println!("Received command {}", command);
    let write_result = stream.write_all(b"+PONG\r\n");
    if write_result.is_err() {
        println!("Unable to write")
    }

    Ok(())
}
