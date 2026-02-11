use dotenv::dotenv;
use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

mod db;
mod schema;

fn main() {
    dotenv().ok();

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Listener: {:?}", listener);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Incoming Stream {http_request:#?}");

    let parts: Vec<&str> = http_request[0].split(" ").collect();
    println!("Get Request: {}", parts[1]);

    println!("Querying database");

    db::api::query_db(parts[1]);
}
