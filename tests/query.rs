use hzip::connection::connection::Connection;
use hzip::handlers::query::FileSystem;
use hzip::handlers::stream::{Algorithm, StreamConfig, Streamable};

#[test]
fn test_check_if_file_exists() {
    let mut c = Connection::new("hzip://localhost:1729?password=hybridzip&archive=test.hz");

    assert!(c.file_exists("/some_random_file.txt".to_string()).unwrap() == false);
}

#[test]
fn test_get_all_files() {
    let mut c = Connection::new("hzip://localhost:1729?password=hybridzip&archive=test.hz");

    c.all_files().unwrap();
}

#[test]
fn test_remove_file() {
    let mut hz = Connection::new("hzip://localhost:1729?password=hybridzip&archive=test.hz");

    let data = "Some data to write to a file".as_bytes();
    hz.write_file(
        data,
        StreamConfig {
            filename: "/test_remove_file.txt".to_string(),
            algorithm: Some(Algorithm::Victini),
            model: None,
        },
    );

    hz.delete_file("/test_remove_file.txt".to_string()).unwrap();

    assert!(hz.file_exists("/test_remove_file.txt".to_string()).unwrap() == false);
}
