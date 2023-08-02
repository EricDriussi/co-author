use std::path::PathBuf;

#[test]
fn should_return_expected_test_authors_file_path() {
    let authors_file_path = PathBuf::from(conf::authors_file());
    assert_eq!(authors_file_path.to_str().unwrap(), "/not_real");
}
