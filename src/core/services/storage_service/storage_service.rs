use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

use crate::core::{AppError, StorageConfig, Validators};
use std::os::unix::fs::PermissionsExt;

pub type StorageResult<T> = Result<T, AppError>;

pub struct StorageService {
    config: StorageConfig,
}

impl StorageService {
    pub fn new(config: StorageConfig) -> Self {
        // Ensure base directory exists
        let _ = std::fs::create_dir_all(&config.base_path);
        Self { config }
    }

    pub fn with_default_config() -> Self {
        Self::new(StorageConfig::default())
    }

    /// Get the full path for a entity's file. For a given file
    pub fn get_entity_file_path(
        &self,
        entity_id: &str,
        entity_dir: &str,
        filename: &str,
    ) -> StorageResult<PathBuf> {
        let entity_dir = Validators::validate_dir_name(entity_dir)?;
        let path = Path::new(&self.config.base_path)
            .join(entity_dir)
            .join(entity_id)
            .join(filename);

        Ok(path)
    }

    /// Get the path for a entity's directory
    pub fn get_entity_directory_path(
        &self,
        entity_id: &str,
        entity_dir: &str,
    ) -> StorageResult<PathBuf> {
        let entity_dir = Validators::validate_dir_name(entity_dir)?;

        let path = Path::new(&self.config.base_path)
            .join(entity_dir)
            .join(entity_id);

        Ok(path)
    }

    /// Create a entity's directory
    ///
    /// The directory is created only if it doesn't exist
    /// This will be created under
    pub async fn create_entity_directory(
        &self,
        entity_id: &str,
        entity_dir: &str,
    ) -> StorageResult<PathBuf> {
        let user_dir = self.get_entity_directory_path(entity_id, entity_dir)?;

        // Create directory with proper permissions
        if !user_dir.exists() {
            fs::create_dir_all(&user_dir)
                .await
                .map_err(|e| AppError::UserDirectoryCreationFailed(e.to_string()))?;

            // Set permissions (Unix specific)
            #[cfg(unix)]
            {
                let mut perms = fs::metadata(&user_dir)
                    .await
                    .map_err(|e| AppError::Io(e.into()))?
                    .permissions();

                perms.set_mode(0o755); // rwxr-xr-x
                fs::set_permissions(&user_dir, perms)
                    .await
                    .map_err(|e| AppError::Io(e.into()))?;
            }
        }

        Ok(user_dir)
    }

    /// Validate file based on config rules
    pub fn validate_file(&self, filename: &str, size: u64) -> StorageResult<()> {
        // Check file size
        if size > self.config.max_file_size {
            return Err(AppError::FileTooLarge(size, self.config.max_file_size));
        }

        // Check file extension
        if let Some(ext) = Path::new(filename).extension() {
            let ext_str = ext.to_string_lossy().to_lowercase();
            if !self
                .config
                .allowed_extensions
                .iter()
                .any(|e| e.to_lowercase() == ext_str)
            {
                return Err(AppError::InvalidFileType(
                    self.config.allowed_extensions.clone(),
                ));
            }
        } else {
            return Err(AppError::InvalidFileType(
                self.config.allowed_extensions.clone(),
            ));
        }

        Ok(())
    }

    /// Generate a safe, unique filename
    pub fn generate_safe_filename(filename: &str) -> String {
        let clean_name = sanitize_filename::sanitize(filename);
        let uuid = Uuid::new_v4();
        format!("{}_{}", uuid, clean_name)
    }

    /// Save a file for a entity
    ///
    /// Returns the newly created safe file name
    pub async fn save_entity_file(
        &self,
        entity_id: &str,
        entity_dir: &str,
        original_filename: &str,
        content: &[u8],
    ) -> StorageResult<String> {
        // Validate file
        self.validate_file(original_filename, content.len() as u64)?;

        // Generate safe filename
        let safe_filename = Self::generate_safe_filename(original_filename);

        // Ensure entity directory exists
        let user_dir = self.create_entity_directory(entity_id, entity_dir).await?;

        // Create full file path
        let file_path = user_dir.join(&safe_filename);

        // Write file
        let mut file = fs::File::create(&file_path)
            .await
            .map_err(|e| AppError::Io(e))?;

        file.write_all(content).await.map_err(|e| AppError::Io(e))?;

        file.flush().await.map_err(|e| AppError::Io(e))?;

        // Set file permissions (Unix specific)
        #[cfg(unix)]
        {
            let mut perms = file
                .metadata()
                .await
                .map_err(|e| AppError::Io(e))?
                .permissions();
            perms.set_mode(0o644); // rw-r--r--
            tokio::fs::set_permissions(&file_path, perms)
                .await
                .map_err(|e| AppError::Io(e))?;
        }

        Ok(safe_filename)
    }

    /// Read one entity's file
    ///
    /// Takes the user_id and file_name to retrieve
    /// Returns file content as bytes vector (Vec<u8>)
    pub async fn read_entity_file(
        &self,
        entity_id: &str,
        entity_dir: &str,
        filename: &str,
    ) -> StorageResult<Vec<u8>> {
        let file_path = self.get_entity_file_path(entity_id, entity_dir, filename)?;

        // Security check: ensure file is within entity directory
        let user_dir = self.get_entity_directory_path(entity_id, entity_dir)?;
        if !file_path.starts_with(&user_dir) {
            return Err(AppError::FileNotFound(filename.to_string()));
        }

        // Check if file exists
        if !file_path.exists() {
            return Err(AppError::FileNotFound(filename.to_string()));
        }

        fs::read(&file_path).await.map_err(|e| AppError::Io(e))
    }

    /// Delete one entity's file
    ///
    /// Takes the file_name to be deleted
    pub async fn delete_entity_file(
        &self,
        entity_id: &str,
        entity_dir: &str,
        filename: &str,
    ) -> StorageResult<()> {
        let file_path = self.get_entity_file_path(entity_id, entity_dir, filename)?;

        // Security check
        let user_dir = self.get_entity_directory_path(entity_id, entity_dir)?;
        if !file_path.starts_with(&user_dir) {
            return Err(AppError::FileNotFound(filename.to_string()));
        }

        if !file_path.exists() {
            return Err(AppError::FileNotFound(filename.to_string()));
        }

        fs::remove_file(&file_path)
            .await
            .map_err(|e| AppError::Io(e))
    }

    /// List all files in a entity's directory
    ///
    /// Returns a vector of all file names
    pub async fn list_entity_files(
        &self,
        entity_id: &str,
        entity_dir: &str,
    ) -> StorageResult<Vec<String>> {
        let user_dir = self.get_entity_directory_path(entity_id, entity_dir)?;

        if !user_dir.exists() {
            return Ok(vec![]);
        }

        let mut entries = fs::read_dir(&user_dir).await.map_err(|e| AppError::Io(e))?;

        let mut files = Vec::new();

        while let Some(entry) = entries.next_entry().await.map_err(|e| AppError::Io(e))? {
            let path = entry.path();
            if path.is_file() {
                if let Some(filename) = path.file_name() {
                    files.push(filename.to_string_lossy().to_string());
                }
            }
        }

        Ok(files)
    }

    /// Get the public URL/path for a file (for serving)
    ///
    /// This can be set in a warp filter in a handler to return the appropriate file
    /// see doc in this file header for how to use it.
    pub fn get_public_path(&self, entity_id: &str, entity_dir: &str, filename: &str) -> String {
        format!("/uploads/{}/{}/{}", entity_dir, entity_id, filename)
    }

    /// Get the absolute filesystem path for a file
    ///
    /// Should be used with care!
    pub fn get_filesystem_path(
        &self,
        entity_id: &str,
        entity_dir: &str,
        filename: &str,
    ) -> StorageResult<PathBuf> {
        self.get_entity_file_path(entity_id, entity_dir, filename)
    }

    /// Delete ALL entity data including their folder and all files
    /// ⚠️ WARNING: This is irreversible! ⚠️
    pub async fn delete_entity_storage_dir(
        &self,
        entity_id: &str,
        entity_dir: &str,
    ) -> StorageResult<()> {
        let user_dir = self.get_entity_directory_path(entity_id, entity_dir)?;

        // Check if directory exists
        if !user_dir.exists() {
            return Err(AppError::UserDirectoryNotFound(format!(
                "User directory does not exist: {:?}",
                user_dir
            )));
        }

        // Security check: Ensure we're only deleting within our base users directory
        let users_base_dir = Path::new(&self.config.base_path).join(entity_dir);
        if !user_dir.starts_with(&users_base_dir) {
            return Err(AppError::Other("Invalid entity directory path".to_string()));
        }

        // Delete the entire entity directory and all contents
        if let Err(e) = fs::remove_dir_all(&user_dir).await {
            return Err(AppError::UserDirectoryDeletionFailed(e.to_string()));
        }
        Ok(())
    }
}
