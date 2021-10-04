use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;

// TODO: Need to improve error handling instead of using unwrap() for everything
// TODO: Add MultiThreaded support
// TODO: Improve styling of base website

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3005").unwrap();

    // Listens for incoming connections on localhost:3005
    // hostname: 127.0.0.1
    // port: 3005
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // println!("Connection established!");
        connection_handler(stream);
    }
}

fn connection_handler(mut stream: TcpStream) {
    
    let mut buffer = [0; 1024];
    
    stream.read(&mut buffer).unwrap();

    // I know this isn't the cleanest way of doing this, but you can add your request URI here to render it.
    let get_response = b"GET / HTTP/1.1\r\n";
    let get_response_2 = b"GET /about HTTP/1.1\r\n";
    let get_response_3 = b"GET /js/index.js HTTP/1.1\r\n";
    let get_response_4 = b"GET /css/style.css HTTP/1.1\r\n";
    let get_response_5 = b"GET /support HTTP/1.1\r\n";
    
    // Once a page has been added to the responses above, add another else/if statement with the location of the HTML, CSS, or JS file. All images neeed to be used in the cloud (ex: AWS) or third-party (ex: imgur).
    let (status_line, filename) = if buffer.starts_with(get_response) {
        ("HTTP/1.1 200 OK", "public/index.html")
    } else if buffer.starts_with(get_response_2) {
        ("HTTP/1.1 200 OK", "public/about.html")
    } else if buffer.starts_with(get_response_3) {
        ("HTTP/1.1 200 OK", "public/js/index.js")
    } else if buffer.starts_with(get_response_4) {
        ("HTTP/1.1 200 OK", "public/css/style.css")
    } else if buffer.starts_with(get_response_5) {
        ("HTTP/1.1 200 OK", "public/support.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "public/error_handling/404.html")
    };

    let return_content = fs::read_to_string(filename).unwrap();
    
    // Handles our response information
    let response = format! (
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        return_content.len(),
        return_content
    );
    
    // This is where our webpage is generated/rendered
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    // Prints request info
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    // Prints info + 404 file
    // println!("Request: {}", response);
}