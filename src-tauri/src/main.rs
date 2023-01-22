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

const NODE_IP: &str = "localhost:5000";

fn main() {
    println!("Listening for incoming connections...");

    let listener = TcpListener::bind("localhost:8080").unwrap();

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
    .invoke_handler(tauri::generate_handler![get_recipient_ip, get_blocks, send_file])
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

    // Get the public key from the client
    let mut client_public_key_str = String::new();
    stream.read_to_string(&mut client_public_key_str).expect("Error reading public client key");

    println!("Client public key: {}", &client_public_key_str);

    let mut file_name = String::new();
    stream.read_to_string(&mut file_name).expect("Error reading file name");
    file_name = file_name.trim().to_string();

    let mut file = match File::create(file_name) {
        Ok(file) => file,
        Err(e) => {
            println!("Error creating file: {}", e);
            return;
        }
    };

    // Read file data from the client and write it to the file
    while let Ok(bytes_read) = stream.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
        match file.write_all(&buffer[..bytes_read]) {
            Ok(_) => (),
            Err(e) => {
                println!("Error writing to file: {}", e);
                return;
            }
        }
    }

    println!("File has been received.");
}


#[tauri::command]
fn send_file(ip: String, file_buffer: Vec<u8>, file_name: String) {
    println!("{}", &ip);
    let mut stream = match TcpStream::connect("localhost:8080") {
        Ok(stream) => stream,
        Err(e) => {
            println!("Error connecting to server: {}", e);
            return;
        }
    };    
    let public_key = get_public_key();
    println!("Public key");
    match stream.write(public_key.as_bytes()) {
        Ok(_) => (),
        Err(e) => {
            println!("Error sending public key: {}", e);
            return;
        }
    }
    println!("File name");
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
