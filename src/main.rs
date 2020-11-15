use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    // create buffer to hold data sent from client
    let mut data = [0 as u8; 50];
    // read data from client into buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo data back to client
            stream.write(&data[0..size]).unwrap();
            true
        }
        Err(_) => {
            print!(
                "An error occurred, terminationg connection with {}",
                stream.peer_addr().unwrap()
            );
            // shutdown the stream
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    // create a tcp server that is bound to port 3333
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    println!("Server listening on port 3333");
    // loop through incoming connections
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                // spawn a new thread to handle each connection
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    // dispose of the server
    drop(listener);
}
