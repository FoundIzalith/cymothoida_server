use std::{
    fs, 
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
}
use cymothoida_server::User;

fn main() {
    println!("Starting new Cymothoida server...");

    let ip = 127.0.0.1:7878;
    let max_users = 8;

    let listener = TcpListener::bind(ip).unwrap();
    let users = User::new(max_users);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        users.execute(|| {
            handleConnection(stream);
        });
    }
}

fn handleConnection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    println!("New connection");

}

//Send messages to connected clients
fn sendMsg(message: String) {

}

fn processCommand(message: String) {

}
