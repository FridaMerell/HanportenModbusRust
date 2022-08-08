use std::io;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use crate::modbus::ModbusCommunication;

pub fn get_connection() -> io::Result<TcpListener> {
    TcpListener::bind("127.0.0.1:5004")
}

pub fn read_port(mut stream: TcpStream) -> ModbusCommunication {
    let mut buffer = [0; 12];

    stream.read(&mut buffer).unwrap();

    ModbusCommunication::parse_incomming(buffer)
}

pub fn respond(mut stream: TcpStream, mut modbus: ModbusCommunication) {}
