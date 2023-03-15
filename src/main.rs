/*************************************************************/
/*      Listens for incoming connections on localhost:1337   */
/*      hostname: 127.0.0.1                                  */
/*      port: 1337                                           */
/*************************************************************/

use std::{
    io::prelude::*,
    net::{
        TcpListener,
        TcpStream,
    },
    fs,
    env
};

use rockbell::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:1337").unwrap();
    let pool = ThreadPool::new(5);

    println!("Rockbell is currently listening on http://127.0.0.1:1337\n");
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            connection_handler(stream);
        });
    }
    
    println!("Shutting down...");
}

fn connection_handler(mut stream: TcpStream) {
    
    let mut buffer = [0; 1024];
    
    stream.read(&mut buffer).unwrap();

    // I know this isn't the cleanest way of doing this, but you can add your request URI here to render it.
    let get_response = b"GET / HTTP/1.1\r\n"; // index.html
    let get_response_2 = b"GET /js/index.js HTTP/1.1\r\n";
    let get_response_3 = b"GET /css/style.css HTTP/1.1\r\n";
    let get_response_4 = b"GET /error_handling/500.html HTTP/1.1\r\n";
    
    // Once a page has been added to the responses above, add another if/else conditional with the location of the HTML, CSS, or JS file. All images neeed to be used in the cloud (ex: AWS) or third-party (ex: imgur)
    let (status_line, filename) = if buffer.starts_with(get_response) {
        ("HTTP/1.1 200 OK", "public/index.html")
    } else if buffer.starts_with(get_response_2) {
        ("HTTP/1.1 200 OK", "public/js/index.js")
    } else if buffer.starts_with(get_response_3) {
        ("HTTP/1.1 200 OK", "public/css/style.css")
    } else if buffer.starts_with(get_response_4) {
        ("HTTP/1.1 500 Internal Server Error", "public/error_handling/500.html") // 500 - Internal Error Code
    } else {
        ("HTTP/1.1 404 NOT FOUND", "public/error_handling/404.html") // 404 - Missing Resource/Page Error Code
    };

    let return_content = fs::read_to_string(filename).unwrap();
    
    // Handles our response information
    let response = format! (
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        return_content.len(),
        return_content
    );

    if status_line == "HTTP/1.1 200 OK" {
        println!("Successfully rendered {} [{}]", filename, status_line);
    } else {
        println!("Unable to render {} [{}]", filename, status_line);
    }

    // This is where our webpage is generated/rendered
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    let args: Vec<String> = env::args().collect();
    let query = &args[1];

    if query == "verbose" {
        // rockbell verbose
        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    } else if query == "debug" {
        // rockbell debug
        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
        println!("Response: {}", response);
    } else if query == "normal" {
        // rockbell normal
    } else {
        println!("Missing query! Available commands: debug, verbose, normal");
        std::process::exit(0);
    }
    
}