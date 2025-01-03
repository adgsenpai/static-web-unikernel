# static-web-unikernel

This project demonstrates how to build and run a Rust-based web server as a unikernel using the OPS toolchain. Unikernels are lightweight, single-purpose virtual machine images that package an application together with only the necessary operating system components, enhancing security, performance, and resource utilization. The Rust application in this project is packaged into a unikernel and run using QEMU within Windows Subsystem for Linux (WSL).

![UniKernel](https://github.com/user-attachments/assets/1834e88e-cf62-4194-85ca-db6e22a21c5d)


## Overview

- **Unikernels** combine the application and minimal OS components, reducing the attack surface and overhead.
- **OPS** is an open-source toolchain used to build and run unikernels.
- **Rust** is utilized for writing a minimal web server due to its performance and memory safety features.

## Motivation

- **Security**: Reduced attack surface since only necessary components are included in the unikernel.
- **Performance**: Minimal context switching and reduced overhead result in improved runtime performance.
- **Portability**: The unikernel image can be easily distributed and run across different platforms.

## Installation

### Prerequisites

1. **Rust**: Ensure you have Rust installed. You can install Rust by running:
   
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **OPS**: Install the OPS toolchain using the following command:

   ```bash
   curl https://ops.city/get.sh -sSfL | sh
   ```

3. **QEMU** (for running the unikernel in an emulated environment):
   
   On Ubuntu:

   ```bash
   sudo apt install qemu
   ```

   For other platforms, check the official QEMU installation guide.

## Building the Web Server

### Step 1: Build the Rust Web Server

1. Clone the repository and navigate to the project directory.
2. Build the Rust executable in release mode:

   ```bash
   cargo build --release
   ```

   This command compiles the Rust web server and produces an optimized binary in the `./target/release/` directory.

### Step 2: Convert to Unikernel

After building the Rust binary, use the OPS toolchain to package it as a unikernel.

1. Build the unikernel image with OPS:

   ```bash
   ops build ./target/release/my_http_server
   ```

   This command will create a bootable unikernel image (e.g., `my_http_server`).

### Step 3: Run the Unikernel

You can run the generated unikernel image using either OPS or QEMU.

#### Run the Unikernel Using OPS

OPS makes it simple to run your unikernel:

```bash
ops run ./target/release/my_http_server
```

#### Run the Unikernel Using QEMU (on WSL)

To run the unikernel under QEMU on Ubuntu inside Windows Subsystem for Linux (WSL), use the following command:

```bash
qemu-system-x86_64 \
-drive file=my_http_server,format=raw \
-nographic \
-netdev user,id=net0,hostfwd=tcp::8080-:8080 \
-device virtio-net,netdev=net0
```

This command starts the unikernel in a virtual machine, with the server listening on port `8080`.

### Step 4: Access the Web Server

Once the unikernel is running, open a web browser and navigate to:

```
http://localhost:8080
```

Alternatively, you can use `curl` to access the server:

```bash
curl http://localhost:8080
```

### Rust Web Server Code

Here is the minimal Rust web server that listens on port 8080 and serves an HTML page displaying basic system statistics (total and used memory) using the `sysinfo` crate:

```rust
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use sysinfo::{System, SystemExt};

fn handle_read(mut stream: &TcpStream) {
    let mut buf = [0u8; 4096];
    match stream.read(&mut buf) {
        Ok(_) => {
            let request_str = String::from_utf8_lossy(&buf);
            println!("{:?}", request_str);
        }
        Err(e) => println!("Unable to read stream: {:?}", e),
    }
}

fn handle_write(mut stream: TcpStream) {
    // Gather system statistics
    let mut sys = System::new_all();
    sys.refresh_all();
    let total_mem = sys.total_memory();
    let used_mem = sys.used_memory();

    // Build an HTML response string with the stats
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
        Err(e) => println!("Failed sending response: {:?}", e),
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
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Unable to connect: {:?}", e);
            }
        }
    }
}
```

### Build the Binary

1. Install Rust:

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Build the Rust executable:

   ```bash
   cargo build --release
   ```

   This will produce the optimized binary in `./target/release/`.

## Converting to a Unikernel with OPS

1. **Install OPS** (if not already installed):

   ```bash
   curl https://ops.city/get.sh -sSfL | sh
   ```

2. **Build the unikernel**:

   ```bash
   ops build ./target/release/my_http_server
   ```

This step packages the Rust binary with only the necessary runtime environment, generating a unikernel image that can be used in QEMU or deployed in cloud environments.

## Performance and Security Considerations

- **Reduced Attack Surface**: Unikernels minimize the OS footprint, limiting the number of attack vectors.
- **Resource Efficiency**: With minimal OS components, unikernels consume fewer resources (RAM, CPU).
- **Isolation**: Running unikernels in virtual machines ensures strong isolation from other services on the same host.

## Future Work

Potential areas for improvement include enhanced networking capabilities, persistent storage support, better security features, and cloud integration. These improvements would make unikernels more robust and suited for production workloads.

## Conclusion

By combining Rust’s performance and memory safety with OPS’s minimal deployment approach, we can build and run highly secure, efficient, and portable unikernels. This project demonstrates the process of creating a simple web server and running it as a unikernel using OPS and QEMU.

## References

- [OPS GitHub](https://github.com/nanovms/ops)
- [Rust Language](https://www.rust-lang.org/)
- [sysinfo Crate](https://crates.io/crates/sysinfo)
- [Unikernels: Library Operating Systems for the Cloud](https://dl.acm.org/doi/10.1145/2517323)
