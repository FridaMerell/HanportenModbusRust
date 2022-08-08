#![allow(dead_code)]
#![allow(unused_variables)]

use std::thread;

use register::Registers;
use tcp::{get_connection, read_port};

mod modbus;
mod register;
mod tcp;

fn main() {
    // Open an SQLite connection and listen for new register values
    thread::spawn(|| loop {
        let register: Registers = Registers::new();
        register.listen_on_serial()
    });

    //main action: Listen on TCP for new modbus requests and send response from registers
    let listener: std::net::TcpListener = get_connection().unwrap();
    let registers: Registers = Registers::new();

    for stream in listener.incoming() {
        let stream: std::net::TcpStream = stream.unwrap();
        let incomming_request: modbus::ModbusCommunication = read_port(stream);
        
    }
}
