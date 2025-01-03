use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

// Add the following to gather system statistics:
use sysinfo::{System, SystemExt};

fn handle_read(mut stream: &TcpStream) {
    let mut buf = [0u8; 4096];
    match stream.read(&mut buf) {
        Ok(_) => {
            let req_str = String::from_utf8_lossy(&buf);
            println!("{}", req_str);
        }
        Err(e) => println!("Unable to read stream: {}", e),
    }
}

fn handle_write(mut stream: TcpStream) {
    // Gather some system stats using sysinfo
    let mut sys = System::new_all();
    sys.refresh_all();

    // For demonstration, gather total memory, used memory, number of CPUs, and average CPU usage
    let total_mem = sys.total_memory();
    let used_mem = sys.used_memory();
    

    // Build an HTML response string that includes the stats
    let response_body = format!(
        r#"
            <html>
                <head>
                    <meta charset="UTF-8">
                    <title>Unikernel Stats</title>
                </head>
                <body>
                    <h1>Hello, Unikernel World!</h1>
                    <p>Here are some system stats:</p>
                    <ul>
                        <li><strong>Total Memory:</strong> {} kB</li>
                        <li><strong>Used Memory:</strong> {} kB</li>                        
                    </ul>
                </body>
            </html>
        "#,
        total_mem, used_mem
    );

    let response = format!(
        "HTTP/1.1 200 OK\r\n\
         Content-Type: text/html; charset=UTF-8\r\n\
         Content-Length: {}\r\n\
         \r\n\
         {}",
        response_body.len(),
        response_body
    );

    match stream.write(response.as_bytes()) {
        Ok(_) => println!("Response sent"),
        Err(e) => println!("Failed sending response: {}", e),
    }
}

fn handle_client(stream: TcpStream) {
    handle_read(&stream);
    handle_write(stream);
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    println!("Welcome to the ADGSTUDIOS - Unikernel World!");
    println!("Listening for connections on port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}
