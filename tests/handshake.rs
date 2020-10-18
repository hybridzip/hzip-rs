use hzip::connection::connection::Connection;
use hzip::handlers::handshake::Handshaker;
use std::net::TcpStream;

#[test]
fn test_handshake() {
    let mut c = Connection::new("localhost:1729".to_string(), "hybridzip".to_string());
    c.stream = Some(TcpStream::connect("localhost:1729").unwrap());
    c.handshake("hybridzip".to_string()).unwrap();
}