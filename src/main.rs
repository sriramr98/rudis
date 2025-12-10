#![allow(unused_imports)]
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

use anyhow::Result;

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                handle_connection(stream)
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    thread::spawn(move || {
        loop {
            if let Err(e) = process_connection(&mut stream) {
                println!("{}", e);
                break;
            }
        }
    });
}

fn process_connection(stream: &mut TcpStream) -> Result<()> {
    let mut buf = [0u8; 512];
    let bytes_read = stream.read(&mut buf)?;

    let command = str::from_utf8(&buf[0..bytes_read])?;

    println!("Received command {}", command);
    stream.write_all(b"+PONG\r\n")?;

    Ok(())
}
