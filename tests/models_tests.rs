use gresources::models::Resource;

#[test]
fn test_resource_creation() {
    let resource = Resource::new("/test/resource".to_string(), "test content".to_string());
    
    assert_eq!(resource.path, "/test/resource");
    assert_eq!(resource.content, Some("test content".to_string()));
    assert_eq!(resource.size, 12); // length of "test content"
    assert_eq!(resource.user_id, 1);
    // Resource has content, so it's not a folder
    assert!(resource.content.is_some());
}

#[test]
fn test_folder_path_extraction() {
    let resource = Resource::new("/folder/subfolder/resource".to_string(), "content".to_string());
    
    assert_eq!(resource.get_folder_path(), "/folder/subfolder");
    // Test that the path ends with "resource"
    assert!(resource.path.ends_with("resource"));
}

#[test]
fn test_root_level_resource() {
    let resource = Resource::new("/resource".to_string(), "content".to_string());
    
    assert_eq!(resource.get_folder_path(), "/");
    // Test that the path ends with "resource"
    assert!(resource.path.ends_with("resource"));
}
