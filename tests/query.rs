use hzip::connection::connection::Connection;
use hzip::handlers::query::FileSystem;

#[test]
fn test_check_if_file_exists() {
    let mut c = Connection::new("hzip://localhost:1729/?password=hybridzip&archive=test.hz");

    assert!(c.check_if_file_exists("/data.txt".to_string()).unwrap() == false);
}