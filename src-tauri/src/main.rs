#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{
    io::{Read, Write},
    net::TcpStream,
};

const NODE_IP: &str = "localhost:5000";

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_reciepient_ip])
        .invoke_handler(tauri::generate_handler![get_blocks])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn send_command(command: &str) -> String {
    let mut stream = TcpStream::connect(NODE_IP).unwrap();
    let mut buf = [0; 1024 * 10];
    stream.write(command.as_bytes()).unwrap();
    stream.read(&mut buf).unwrap();
    String::from_utf8_lossy(&buf[..]).to_string()
}

#[tauri::command]
fn get_reciepient_ip(public_key: &str) -> String {
    let message = "get_peer ".to_string() + public_key;
    send_command(&message)
}

#[tauri::command]
fn get_blocks() -> String {
    send_command("get_blocks")
}
