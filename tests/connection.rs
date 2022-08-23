use std::net::TcpListener;

#[cfg(test)]
#[test]
fn port_listening() {
    let listener = TcpListener::bind("127.0.0.1:1337").unwrap();
    assert_eq!(listener.local_addr().unwrap().port(), 1337);
}