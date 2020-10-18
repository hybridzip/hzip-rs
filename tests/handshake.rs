use hzip::connection::connection::Connection;
use hzip::handlers::session::SessionManager;
use std::net::TcpStream;

#[test]
fn test_handshake() {
    let mut c = Connection::new("localhost:1729".to_string(), "hybridzip".to_string());
    c.refresh_session();
}