# Rockbell

Rockbell is a Multithreaded HTTP server built in Rust for local development. 

Rockbell is a barebones HTTP (GET) server that renders HTML, CSS, and JS files. Feel free to check out the `/public` directory to see what the server renders when running with cargo.

## Usage
Running this command will boot up the server and automatically connect to port `1337` in normal mode.
```bash
cargo run -- normal
```

If you would like to run a test prior to starting the server, use this command:
```bash
cargo test
```

# Command Line Options

Starts the server in normal mode (no messages other than status codes and errors are printed)
```bash
cargo run -- normal
```

Starts the server in verbose mode (prints user-agent information, request status, worker status, and other basic responses)
```bash
cargo run -- verbose
```

Starts the server in debug mode (every message, including user-agent information, request status, worker status, public source code, and much more is printed)
```bash
cargo run -- debug
```