use hzip::connection::connection::Connection;
use hzip::handlers::query::Queryable;

#[test]
fn test_check_if_file_exists() {
    let mut c = Connection::new(
        "localhost:1729".to_string(),
        "hybridzip".to_string(),
        "test.hz".to_string()
    );

    assert!(c.check_if_file_exists("/data.txt".to_string()).unwrap() == false);
}