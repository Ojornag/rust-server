use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;

const BUF_SIZE: usize = 65536;
const ROOT_PATH: &str = "/srv/http";

struct Response {
    response_code: String,
    server: String,
    content_length: usize,
    content_type: String,
    data: Vec<u8>
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80")?;

    for stream in listener.incoming() {
        let mut stream = stream?;

        // Read request into buffer
        let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
        stream.read(&mut buf[..])?;

        // Handle request
        let res = handle_request(std::str::from_utf8(&buf[..]).unwrap());

        // Parse response into text
        let headers = format!("HTTP/1.1 {}\nServer: {}\nContent-Length: {}\nContent-Type: {}\n\r\n",
            res.response_code,
            res.server,
            res.content_length,
            res.content_type,
        );

        // Write response to stream
        let res_bytes = [headers.as_bytes(), &res.data[..]].concat();
        stream.write(&res_bytes[..])?;
    }

    Ok(())
}

fn handle_request(req: &str) -> Response {
    // Create base response
    let mut res = Response {
        response_code: "200 OK".to_owned(),
        server: "Ojornag's rust server".to_owned(),
        content_length: 0,
        content_type: "text/html".to_owned(),
        data: vec![]
    };

    // Read file
    let file_path = format!("{ROOT_PATH}/index.html");
    let file = match fs::read(file_path) {
        Ok(file) => file,
        Err(_) => {
            // Return 404
            res.response_code = "404 Not Found".to_owned();
            return res;
        }
    };

    // Add data to response and return it
    res.content_length = file.len();
    res.data = file;
    return res;
}
