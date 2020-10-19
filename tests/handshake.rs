use hzip::connection::connection::Connection;
use hzip::handlers::session::SessionManager;

#[test]
fn test_handshake() {
    let mut c = Connection::new("hzip://localhost:1729/?password=hybridzip&archive=test.hz");

    c.refresh_session().unwrap();
    c.refresh_session().unwrap();
}