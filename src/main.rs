use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};
use std::thread;
use uuid::Uuid;
use chrono::prelude::*;
extern crate chrono;
extern crate base64;

// A basic TCP server.
// Take any data from the socket and dump it to standard out.
// The STDOUT data from M I H N O is to then be fed into
// the desired backend/consumer etc.
// This program does not include TLS and expects
// that the 0.0.0.0:3975 can be replaced with 127.0.0.1:3875
// and a TLS layer in front of it, or otherwise
// any needed encryption is handled elsewhere etc.

// Collect anything we get sent.
fn harvest_client(mut stream: &TcpStream,txid: Uuid) {
    let readu: DateTime<Utc> = Utc::now();
    let mut buf = [0u8 ;4096];
    match stream.read(&mut buf) {
        Ok(_) => {
//            let req_str = String::from_utf8_lossy(&buf);
//          If you want to use raw data without encoding, comment out
//          out the below base64, comment the above in, and use req_str instead.
            let b64_str = base64::encode(&buf);
            let req_src = stream.peer_addr().unwrap();
            println!("{} {} {} {}", readu, txid, req_src, b64_str);
            },
        Err(e) => println!("Unable to read stream: {}", e),
    }
}

// Send back a fixed response or print the error.
// In this case we include an HTTP response
// along with a static marker hash. This hash
// is a SHA256 with input from a PRNG
// that is being used to label this build.
// Do what you want with it, make the response
// appropriate for your situation etc.
fn response_client(mut stream: TcpStream,txid: Uuid) {
    let writeo: DateTime<Utc> = Utc::now();
    let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n5aedbcfdfd6cf03dc48f8af74b887bad46bb559df5daddacb9e960d7af12269e\r\n";
    match stream.write(response) {
        Ok(_) => println!("{} {} {}", writeo, txid, "Response sent _<---_ end transaction"),
        Err(e) => println!("{} {} {} {}", writeo, txid, e, "Failed sending response _<-!_ end transaction"),
    }
}

fn transaxor(stream: TcpStream,txid: Uuid) {
    harvest_client(&stream,txid);
    response_client(stream,txid);
}

fn main() {
    let initu = Utc::now();
    let listener = TcpListener::bind("0.0.0.0:3975").unwrap();
    println!("{} {}", initu, "M_I_H_N_O - Listening for connections on port 3975");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    let recvu = Utc::now();
                    let txid = Uuid::new_v4();
                    let str_src = stream.peer_addr().unwrap();
                    println!("{} {} {} {}", recvu, txid, " _--->_ start transaction with ", str_src);
                    transaxor(stream,txid)
                });
            }
            Err(e) => {
                let inite = Utc::now();
                println!("{} {} {}", inite, "Unable to connect: {}", e);
            }
        }
    }
}
