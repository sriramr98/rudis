#![allow(unused_imports)]
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

mod resp;

use anyhow::Result;

use crate::resp::parser::parse_resp;

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
    
    let request = parse_resp(command)?;

    request.validate()?;

    let resp = request.execute()?;

    stream.write_all(&resp.encode())?;
    Ok(())
}
