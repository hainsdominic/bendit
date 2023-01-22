#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs::File;
use std::net::TcpListener;
use std::thread;
use std::{
    fs,
    io::{Read, Write},
    net::TcpStream,
    path::Path,
};

const NODE_IP: &str = "192.168.11.76:5000";

fn main() {
    println!("Listening for incoming connections...");

    let listener = TcpListener::bind("192.168.11.183:8080").unwrap();

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

    preload();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_recipient_ip,
            get_blocks,
            send_file,
            get_download_files
        ])
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
    let public_key = fs::read_to_string(path).unwrap_or("".to_string());
    public_key
}

fn file_reception_loop(mut stream: TcpStream) {
    let mut buffer = [0; 2_000_000];

    stream.read(&mut buffer).unwrap();

    // Get the public key from the client
    let message = String::from_utf8_lossy(&buffer[..]).to_string();

    // println!("message: {}", message);

    let public_key = message.split("\r1\n\r\n").next().unwrap().to_string();
    println!("public_key: {}", public_key);
    let mut file_name = message.split("\r1\n\r\n").nth(1).unwrap().to_string();
    stream
        .read_to_string(&mut file_name)
        .expect("Error reading file name");
    file_name = file_name.trim().to_string();
    println!("file_name: {}", file_name);

    let path = Path::new("../downloads").join(&file_name);
    println!("path: {}", path.display());

    let mut file = match File::create(&path) {
        Ok(file) => file,
        Err(e) => {
            println!("Error creating file: {}", e);
            return;
        }
    };

    let file_data = message.split("\r1\n\r\n").nth(2).unwrap().to_string();

    match file.write_all(file_data.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", file_name, why),
        Ok(_) => println!("successfully wrote to {}", file_name),
    }

    println!("File has been received.");
}

#[tauri::command]
fn get_download_files() -> Vec<String> {
    let path = Path::new("../downloads");
    let mut files = Vec::new();
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        files.push(path.display().to_string());
    }
    files
}

#[tauri::command]
fn send_file(ip: String, file_buffer: Vec<u8>, file_name: String) {
    // let ip = ip.split(":").next().unwrap();
    // let ip = ip.trim_end_matches(char::from(0));
    let mut stream = match TcpStream::connect(ip.to_string() + ":8080") {
        // let mut stream = match TcpStream::connect("192.168.11.200".to_string() + ":8080") {
        Ok(stream) => stream,
        Err(e) => {
            println!("Error connecting to server {}: {}", ip, e);
            return;
        }
    };
    let public_key = get_public_key().to_string() + "\r1\n\r\n";
    println!("Public key");
    match stream.write(public_key.as_bytes()) {
        Ok(_) => (),
        Err(e) => {
            println!("Error sending public key: {}", e);
            return;
        }
    }
    println!("File name");
    let file_name = file_name + "\r1\n\r\n";
    match stream.write(file_name.as_bytes()) {
        Ok(_) => (),
        Err(e) => {
            println!("Error sending file name: {}", e);
            return;
        }
    }
    println!("File buffer");
    match stream.write(&file_buffer) {
        Ok(_) => (),
        Err(e) => {
            println!("Error sending file buffer: {}", e);
            return;
        }
    }

    match stream.write("\r1\n\r\n".as_bytes()) {
        Ok(_) => (),
        Err(e) => {
            println!("Error sending file buffer: {}", e);
            return;
        }
    }

    println!("File has been sent.");
}

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
