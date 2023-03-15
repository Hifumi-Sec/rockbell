# Rockbell

Rockbell is a Multithreaded HTTP server built in Rust for local development. 

Rockbell is a barebones HTTP (GET) server that renders HTML, CSS, and JS files. Feel free to check out the `/public` directory to see what the server renders when running with cargo.

## Installation
You can install the crate via `Cargo` or by adding this line to your `Cargo.toml` file:
```bash
rockbell = "0.0.1"
```

## Usage
Running this command will boot up the server and automatically connect to port `1337` in normal mode.
```bash
rockbell normal
```

## Command Line Options

Starts the server in normal mode (no messages other than status codes and errors are printed)
```bash
rockbell normal
```

Starts the server in verbose mode (prints user-agent information, request status, worker status, and other basic responses)
```bash
rockbell verbose
```

Starts the server in debug mode (every message, including user-agent information, request status, worker status, public source code, and much more is printed)
```bash
rockbell debug
```