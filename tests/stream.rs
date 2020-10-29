use hzip::connection::connection::Connection;
use hzip::handlers::query::FileSystem;
use hzip::handlers::stream::{Algorithm, StreamConfig, Streamable};
use std::fs::File;
use std::fs;

#[test]
fn test_write_file_without_model() {
    let mut hz = Connection::new("hzip://localhost:1729?password=hybridzip&archive=test.hz");

    let data = "Some data to write to a file".as_bytes();

    hz.write_file(
        data,
        StreamConfig {
            filename: Some("/test_write_file_without_model.txt".to_string()),
            algorithm: Some(Algorithm::Victini),
            ..Default::default()
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
            filename: Some("/test_write_file_without_model.txt".to_string()),
            ..Default::default()
        },
    )
    .unwrap();

    assert_eq!(
        "Some data to write to a file".to_string(),
        std::str::from_utf8(&buf).unwrap()
    );
}

#[test]
fn test_train_model() {
    let mut hz = Connection::new("hzip://localhost:1729?password=hybridzip&archive=test.hz");

    let mut file = File::open("/home/supercmmetry/Documents/datasets/text/enwik9").unwrap();
    let metadata = fs::metadata("/home/supercmmetry/Documents/datasets/text/enwik9").unwrap();

    hz.train_model(file, StreamConfig {
        algorithm: Some(Algorithm::Victini),
        model: Some("/victini/enwik9".to_string()),
        stream_size: Some(metadata.len() as usize),
        ..Default::default()
    }).unwrap();
}

#[test]
fn test_read_model() {
    let mut hz = Connection::new("hzip://localhost:1729?password=hybridzip&archive=test.hz");

    let mut file = File::create("victini.enwik9.hm").unwrap();

    hz.read_model(file, StreamConfig {
        algorithm: Some(Algorithm::Victini),
        model: Some("/victini/enwik9".to_string()),
        ..Default::default()
    }).unwrap();
}

#[test]
fn test_write_model() {
    let mut hz = Connection::new("hzip://localhost:1729?password=hybridzip&archive=test.hz");

    let mut file = File::open("/home/supercmmetry/Projects/hzip-research/Models/victini.enwik9.hm").unwrap();

    hz.write_model(file, StreamConfig {
        algorithm: Some(Algorithm::Victini),
        model: Some("/victini/enwik9".to_string()),
        ..Default::default()
    }).unwrap();
}
