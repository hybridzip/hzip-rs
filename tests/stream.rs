use hzip::connection::connection::Connection;
use hzip::handlers::query::FileSystem;
use hzip::handlers::stream::{Algorithm, StreamConfig, Streamable};

#[test]
fn test_write_file_without_model() {
    let mut hz = Connection::new("hzip://localhost:1729?password=hybridzip&archive=test.hz");

    let data = "Some data to write to a file".as_bytes();
    hz.write_file(
        data,
        StreamConfig {
            filename: "/test_write_file_without_model.txt".to_string(),
            algorithm: Some(Algorithm::Victini),
            model: None,
        },
    );

    assert!(
        hz.file_exists("/test_write_file_without_model.txt".to_string())
            .unwrap()
            == true
    );

    let mut buf: Vec<u8> = vec![];

    hz.read_file(
        &mut buf,
        StreamConfig {
            filename: "/test_write_file_without_model.txt".to_string(),
            algorithm: None,
            model: None,
        },
    )
    .unwrap();

    assert_eq!(
        "Some data to write to a file".to_string(),
        std::str::from_utf8(&buf).unwrap()
    );
}
