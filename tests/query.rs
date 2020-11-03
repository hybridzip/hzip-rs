use hzip::connection::connection::Connection;
use hzip::handlers::query::Queryable;
use hzip::handlers::stream::{Algorithm, StreamConfig, Streamable};

#[test]
fn test_check_if_file_exists() {
    let mut c = Connection::new("hzip://localhost:1729?password=hybridzip&archive=test.hz");

    assert!(c.file_exists("/some_random_file.txt".to_string()).unwrap() == false);
}

#[test]
fn test_get_some_files() {
    let mut c = Connection::new("hzip://localhost:1729?password=hybridzip&archive=test.hz");

    dbg!(c.list_fs("/".to_string()).unwrap());
}

#[test]
fn test_remove_file() {
    let mut hz = Connection::new("hzip://localhost:1729?password=hybridzip&archive=test.hz");

    let data = "Some data to write to a file".as_bytes();

    if hz.file_exists("/test_remove_file.txt".to_string()).unwrap() {
        hz.delete_file("/test_remove_file.txt".to_string()).unwrap();
    }

    hz.write_file(
        data,
        StreamConfig {
            filename: Some("/test_remove_file.txt".to_string()),
            algorithm: Some(Algorithm::Victini),
            ..Default::default()
        },
    )
    .unwrap();

    hz.delete_file("/test_remove_file.txt".to_string()).unwrap();

    assert!(hz.file_exists("/test_remove_file.txt".to_string()).unwrap() == false);
}

#[test]
fn test_get_mem_usage() {
    let mut hz = Connection::new("hzip://localhost:1729?password=hybridzip&archive=test.hz");

    dbg!(hz.get_mem_usage().unwrap());
}
