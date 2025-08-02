use gresources::validation::{normalize_path, validate_content, validate_path};

#[test]
fn test_path_validation() {
    // Valid paths
    assert!(validate_path("/resource").is_ok());
    assert!(validate_path("/folder/resource").is_ok());
    assert!(validate_path("/").is_ok());

    // Invalid paths
    assert!(validate_path("").is_err());
    assert!(validate_path("no-leading-slash").is_err());

    // Test max depth (5 levels)
    assert!(validate_path("/a/b/c/d/e").is_ok());
    assert!(validate_path("/a/b/c/d/e/f").is_err());
}

#[test]
fn test_content_validation() {
    assert!(validate_content("small content").is_ok());

    // Test max size (5MB)
    let large_content = "x".repeat(5 * 1024 * 1024 + 1);
    assert!(validate_content(&large_content).is_err());
}

#[test]
fn test_path_normalization() {
    assert_eq!(normalize_path("/path/"), "/path");
    assert_eq!(normalize_path("/"), "/");
    assert_eq!(normalize_path("/path/to/resource"), "/path/to/resource");
}
