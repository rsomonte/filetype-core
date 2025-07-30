//! ufile-core: Pure file type identification logic for use in CLI and Wasm frontends.

mod magicnums;
pub use magicnums::get_magic_numbers;

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Structured information about a detected file type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileInfo {
    /// The path to the file
    pub path: PathBuf,
    /// A human-readable description of the file type
    pub description: String,
    /// Whether this is a directory
    pub is_directory: bool,
    /// File size in bytes (None for directories)
    pub size: Option<u64>,
}

/// Error types for file processing operations.
#[derive(Debug, thiserror::Error)]
pub enum FileProcessingError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Path does not exist: {0}")]
    PathNotFound(PathBuf),
    #[error("Directory traversal error: {0}")]
    WalkDir(#[from] walkdir::Error),
}

/// Identify the file type from a byte slice.
/// Returns Some(FileInfo) if recognized, or None otherwise.
pub fn identify_from_bytes(bytes: &[u8]) -> Option<FileInfo> {
    for entry in get_magic_numbers() {
        if bytes.len() >= entry.offset + entry.magic.len()
            && &bytes[entry.offset..entry.offset + entry.magic.len()] == entry.magic
        {
            return Some(FileInfo {
                path: PathBuf::new(),
                description: entry.description.to_string(),
                is_directory: false,
                size: Some(bytes.len() as u64),
            });
        }
    }
    // Fallback to infer if no custom magic matched
    if let Some(kind) = infer::get(bytes) {
        return Some(FileInfo {
            path: PathBuf::new(),
            description: kind.mime_type().to_string(),
            is_directory: false,
            size: Some(bytes.len() as u64),
        });
    }
    None
}

/// Create a FileInfo for a directory.
/// Helper function for multi-file operations.
fn create_directory_info<P: AsRef<Path>>(path: P) -> FileInfo {
    FileInfo {
        path: path.as_ref().to_path_buf(),
        description: "Directory".to_string(),
        is_directory: true,
        size: None,
    }
}

/// Create a FileInfo for a file by reading it and identifying its type.
/// Helper function for multi-file operations.
fn identify_file_from_path<P: AsRef<Path>>(path: P) -> Result<FileInfo, FileProcessingError> {
    let path = path.as_ref();
    let metadata = fs::metadata(path)?;
    
    if metadata.is_dir() {
        return Ok(create_directory_info(path));
    }

    let bytes = fs::read(path)?;
    let description = if let Some(info) = identify_from_bytes(&bytes) {
        info.description
    } else {
        "Unknown file type".to_string()
    };
    
    Ok(FileInfo {
        path: path.to_path_buf(),
        description,
        is_directory: false,
        size: Some(metadata.len()),
    })
}

/// Process multiple files and/or directories.
/// Returns a vector of FileInfo for all processed items.
pub fn identify_multiple<P: AsRef<Path>>(paths: &[P]) -> Result<Vec<FileInfo>, FileProcessingError> {
    let mut results = Vec::new();
    
    for path in paths {
        let path = path.as_ref();
        
        if !path.exists() {
            return Err(FileProcessingError::PathNotFound(path.to_path_buf()));
        }
        
        let file_info = identify_file_from_path(path)?;
        results.push(file_info);
    }
    
    Ok(results)
}

/// Recursively process a directory and all its contents.
/// Returns a vector of FileInfo for all files and subdirectories found.
pub fn identify_recursive<P: AsRef<Path>>(path: P) -> Result<Vec<FileInfo>, FileProcessingError> {
    let path = path.as_ref();
    
    if !path.exists() {
        return Err(FileProcessingError::PathNotFound(path.to_path_buf()));
    }
    
    let mut results = Vec::new();
    
    for entry in WalkDir::new(path) {
        let entry = entry?;
        let entry_path = entry.path();
        
        let file_info = identify_file_from_path(entry_path)?;
        results.push(file_info);
    }
    
    Ok(results)
}

/// Process multiple paths, recursively walking directories.
/// Returns a vector of FileInfo for all processed items.
pub fn identify_multiple_recursive<P: AsRef<Path>>(paths: &[P]) -> Result<Vec<FileInfo>, FileProcessingError> {
    let mut results = Vec::new();
    
    for path in paths {
        let path = path.as_ref();
        
        if !path.exists() {
            return Err(FileProcessingError::PathNotFound(path.to_path_buf()));
        }
        
        if path.is_dir() {
            results.extend(identify_recursive(path)?);
        } else {
            let file_info = identify_file_from_path(path)?;
            results.push(file_info);
        }
    }
    
    Ok(results)
}

/// Filter results to only include files (not directories).
pub fn filter_files(results: Vec<FileInfo>) -> Vec<FileInfo> {
    results.into_iter().filter(|info| !info.is_directory).collect()
}

/// Filter results to only include directories.
pub fn filter_directories(results: Vec<FileInfo>) -> Vec<FileInfo> {
    results.into_iter().filter(|info| info.is_directory).collect()
}

/// Group results by file type description.
pub fn group_by_type(results: Vec<FileInfo>) -> std::collections::HashMap<String, Vec<FileInfo>> {
    let mut grouped = std::collections::HashMap::new();
    
    for file_info in results {
        grouped.entry(file_info.description.clone())
            .or_insert_with(Vec::new)
            .push(file_info);
    }
    
    grouped
}

/// Identify the file types from multiple byte slices.
/// Returns a vector of FileInfo for all processed items.
pub fn identify_many_bytes<'a, I>(files: I) -> Vec<FileInfo>
where
    I: IntoIterator<Item = &'a [u8]>,
{
    files
        .into_iter()
        .map(|bytes| identify_from_bytes(bytes).unwrap_or(FileInfo {
            path: PathBuf::new(),
            description: "Unknown file type".to_string(),
            is_directory: false,
            size: Some(bytes.len() as u64),
        }))
        .collect()
}
