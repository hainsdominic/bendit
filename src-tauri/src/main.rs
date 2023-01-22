#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{net::TcpStream, io::{Read, Write}};
// use std::path::Path;

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

// fn get_private_key() -> String {
//     let path = Path::new("private.key");
//     let private_key = fs::read_to_string(path).unwrap();
//     private_key
// }

// fn get_public_key() -> String {
//     let path = Path::new("public.key");
//     let public_key = fs::read_to_string(path).unwrap();
//     public_key
// }


// fn generate_keypair() -> (Vec<u8>, Vec<u8>) {
//     let key = Rsa::generate(2048).unwrap();
//     let private_key = key.private_key_to_pem().unwrap();
//     let public_key = key.public_key_to_pem().unwrap();
//     (public_key, private_key)
// }


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
