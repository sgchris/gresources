use anyhow::{Result, anyhow};

const MAX_RESOURCE_NAME_LENGTH: usize = 100;
const MAX_RESOURCE_SIZE: usize = 5 * 1024 * 1024; // 5MB
const MAX_FOLDER_DEPTH: usize = 5;

pub fn validate_path(path: &str) -> Result<()> {
    if path.is_empty() {
        return Err(anyhow!("Path cannot be empty"));
    }

    if !path.starts_with('/') {
        return Err(anyhow!("Path must start with '/'"));
    }

    // Check folder depth
    let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    let depth = segments.len();
    if depth > MAX_FOLDER_DEPTH {
        return Err(anyhow!("Maximum folder depth is {}", MAX_FOLDER_DEPTH));
    }

    // Validate each path segment
    for segment in segments {
        if segment.len() > MAX_RESOURCE_NAME_LENGTH {
            return Err(anyhow!("Resource name cannot exceed {} characters", MAX_RESOURCE_NAME_LENGTH));
        }
        
        if segment.contains("..") || segment.contains('\0') {
            return Err(anyhow!("Invalid characters in path"));
        }
        
        // Check for reserved characters that might cause issues
        if segment.chars().any(|c| matches!(c, '<' | '>' | ':' | '"' | '|' | '?' | '*')) {
            return Err(anyhow!("Path contains reserved characters"));
        }
    }

    Ok(())
}

pub fn validate_content(content: &str) -> Result<()> {
    if content.len() > MAX_RESOURCE_SIZE {
        return Err(anyhow!("Content size cannot exceed {} bytes", MAX_RESOURCE_SIZE));
    }
    
    Ok(())
}

pub fn normalize_path(path: &str) -> String {
    // Remove trailing slashes except for root
    if path == "/" {
        path.to_string()
    } else {
        path.trim_end_matches('/').to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_path() {
        assert!(validate_path("/valid/path").is_ok());
        assert!(validate_path("/").is_ok());
        assert!(validate_path("").is_err());
        assert!(validate_path("no-leading-slash").is_err());
        
        // Test max depth
        let deep_path = "/a/b/c/d/e/f"; // 6 levels - should fail
        assert!(validate_path(deep_path).is_err());
        
        let max_depth_path = "/a/b/c/d/e"; // 5 levels - should pass
        assert!(validate_path(max_depth_path).is_ok());
    }

    #[test]
    fn test_validate_content() {
        assert!(validate_content("small content").is_ok());
        
        let large_content = "x".repeat(MAX_RESOURCE_SIZE + 1);
        assert!(validate_content(&large_content).is_err());
    }

    #[test]
    fn test_normalize_path() {
        assert_eq!(normalize_path("/path/"), "/path");
        assert_eq!(normalize_path("/"), "/");
        assert_eq!(normalize_path("/path/to/resource"), "/path/to/resource");
    }
}
