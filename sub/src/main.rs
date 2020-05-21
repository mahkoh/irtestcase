use std::path::Path;

#[test]
fn test() {
    assert!(Path::new("file").exists());
}