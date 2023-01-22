#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{
    io::{Read, Write},
    net::TcpStream, path::Path, fs,
};

const NODE_IP: &str = "localhost:5000";


fn main() {
    preload();
    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_recipient_ip, get_blocks])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
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
    let public_key = fs::read_to_string(path).unwrap();
    public_key
}

//send a React File object to the server
// #[tauri::command]
// fn send_file(ip: String, 


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
