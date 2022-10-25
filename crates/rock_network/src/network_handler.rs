use log::info;
use std::{
    io::{BufReader, Write, BufRead},
    net::{TcpListener, TcpStream},
};

pub fn start(addr: &str) -> anyhow::Result<()> {
    let listener = TcpListener::bind(addr).expect("Couldn't start server");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        info!("Connection established");

        handle_connection(stream);
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("Request: {:#?}", http_request);

    let response = "HTTP/1.1 200 OK\r\n\r\nTest";
    stream.write_all(response.as_bytes()).unwrap();
}
