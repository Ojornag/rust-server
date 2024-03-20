use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

const BUF_SIZE: usize = 65536;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80")?;

    for stream in listener.incoming() {
        let mut stream = stream?;

        // Read request into buffer
        let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
        stream.read(&mut buf[..])?;

        // Write response
        let response: &[u8] = "HTTP/1.1 200 OK\n".as_bytes();
        stream.write(response)?;
    }

    Ok(())
}
