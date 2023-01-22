#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use base64::{self, Engine};
use std::fs::File;
use std::net::TcpListener;
use std::thread;
use std::{
    fs,
    io::{Read, Write},
    net::TcpStream,
    path::Path,
};

const NODE_IP: &str = "localhost:5000";

fn main() {
    preload();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_recipient_ip,
            get_blocks,
            send_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    thread::spawn(move || {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(move || {
                        file_reception_loop(stream);
                    });
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    });
}

fn preload() {
    let command = "add_peer ".to_string() + &get_public_key();
    send_command(&command);
}

fn send_command(command: &str) -> String {
    let mut stream = TcpStream::connect(NODE_IP).unwrap();
    let mut buf = [0; 1024 * 10];
    stream.write(command.as_bytes()).unwrap();
    stream.read(&mut buf).unwrap();
    String::from_utf8_lossy(&buf[..]).to_string()
}

// fn get_private_key() -> String {
//     let path = Path::new("../asset/private.key");
//     let private_key = fs::read_to_string(path).unwrap();
//     private_key
// }

fn get_public_key() -> String {
    let path = Path::new("asset/public.key");
    let public_key = fs::read_to_string(path).unwrap_or("".to_string());
    public_key
}

fn file_reception_loop(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    let mut client_public_key = vec![];

    // Get the public key from the client
    while let Ok(bytes_read) = stream.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
        client_public_key.extend_from_slice(&buffer[..bytes_read]);
        if client_public_key.ends_with(b"\n") {
            break;
        }
    }
    let client_public_key_str = String::from_utf8(client_public_key).unwrap();

    // Get the file name from the client
    stream.read(&mut buffer).unwrap();

    let file_name = String::from_utf8_lossy(&buffer).trim().to_string();

    // Open a file with the same name
    let mut file = File::create(file_name).unwrap();

    // Read file data from the client and write it to the file
    while let Ok(bytes_read) = stream.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
        file.write_all(&buffer[..bytes_read]).unwrap();
    }

    println!("File has been received.");
}

#[tauri::command]
fn send_file(ip: String, fileBuffer: &str) {
    println!("{}", fileBuffer);
}

// fn generate_keypair() -> (Vec<u8>, Vec<u8>) {
//     let key = Rsa::generate(2048).unwrap();
//     let private_key = key.private_key_to_pem().unwrap();
//     let public_key = key.public_key_to_pem().unwrap();
//     (public_key, private_key)
// }

// call a tcp server
#[tauri::command]
fn get_recipient_ip(public_key: String) -> String {
    let message = "get_peer ".to_string() + &public_key;
    send_command(&message)
}

#[tauri::command]
fn get_blocks() -> String {
    send_command("get_blocks")
}
