#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{net::TcpStream, io::{Read, Write}};

const NODE_IP: &str = "localhost:5000";

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![say_hello])
        .invoke_handler(tauri::generate_handler![get_reciepient_ip])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn say_hello() {
    println!("Hello from Rust!");
}

// call a tcp server
#[tauri::command]
fn get_reciepient_ip(public_key: &str) -> String {
    let mut stream = TcpStream::connect(NODE_IP).unwrap();
    let mut buf = [0; 1024];
    let message = "get_peer ".to_string() + public_key;
    stream.write(message.as_bytes()).unwrap();
    stream.read(&mut buf).unwrap();
    println!("Received: {}", String::from_utf8_lossy(&buf[..]));

    return String::from_utf8_lossy(&buf[..]).to_string();
}

