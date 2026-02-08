use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct StorageConfig {
    pub base_path: String,
    pub max_file_size: u64,              // in bytes
    pub allowed_extensions: Vec<String>, // e.g., ["jpg", "png", "jpeg", "gif"]
}

impl Default for StorageConfig {
    /// Create a default storage configuration
    fn default() -> Self {
        Self {
            base_path: "./uploads".to_string(),
            max_file_size: 10 * 1024 * 1024, // 10MB
            allowed_extensions: vec![
                "jpg".to_string(),
                "jpeg".to_string(),
                "png".to_string(),
                "gif".to_string(),
            ],
        }
    }
}
