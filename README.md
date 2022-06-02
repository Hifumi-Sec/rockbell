# Rockbell

Rockbell is a Multithreaded HTTP server built in Rust for local development. 

Rockbell is a barebones HTTP (GET) server that renders HTML, CSS, and JS files. Feel free to check out the `/public` directory to see what the server renders when running with cargo.

## Usage
Running this command will boot up the server and automatically connect to port `1337`.
```bash
$ cargo run
```

If you would like to run a test prior to starting the server, use this command:
```bash
$ cargo test
```
