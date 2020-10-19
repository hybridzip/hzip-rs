use hzip::connection::connection::Connection;
use hzip::handlers::query::FileSystem;

#[test]
fn test_check_if_file_exists() {
    let mut c = Connection::new("hzip://localhost:1729/?password=hybridzip&archive=test.hz");

    assert!(c.file_exists("/data.txt".to_string()).unwrap() == false);
}

#[test]
fn test_get_all_files() {
    let mut c = Connection::new("hzip://localhost:1729/?password=hybridzip&archive=test.hz");

    assert!(c.all_files().unwrap() == vec![] as Vec<String>);
}
