use std::{net::{TcpListener, TcpStream}, io::{BufReader}};
use log::{info};

pub fn start(addr: &str) -> anyhow::Result<()> {
    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        info!("Connection established");

        handle_connection(stream);
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let _buf_reader = BufReader::new(&mut stream);
}