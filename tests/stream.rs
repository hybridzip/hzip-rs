use hzip::connection::connection::Connection;
use hzip::handlers::query::FileSystem;
use hzip::handlers::stream::{Streamable, StreamConfig, Algorithm};

#[test]
fn test_write_file_without_model() {
    let mut hz = Connection::new("hzip://localhost:1729/?password=hybridzip&archive=test.hz");

    let data = "Some data to write to a file".as_bytes();
    hz.write_file(data, StreamConfig {
        filename: "/sample.txt".to_string(),
        algorithm: Algorithm::Victini,
        model: None,
    });

    assert!(hz.file_exists("/sample.txt".to_string()).unwrap() == true);
    assert_eq!(vec!["/sample.txt".to_string()], hz.all_files().unwrap());
}