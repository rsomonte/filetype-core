//! filetype-core: Pure file type identification logic for use in CLI and Wasm frontends.

mod magicnums;
pub use magicnums::get_magic_numbers;

/// Structured information about a detected file type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileInfo {
    /// A human-readable description of the file type
    pub description: String,
}

/// Identify the file type from a byte slice.
/// Returns Some(FileInfo) if recognized, or None otherwise.
pub fn identify_from_bytes(bytes: &[u8]) -> Option<FileInfo> {
    for entry in get_magic_numbers() {
        if bytes.len() >= entry.offset + entry.magic.len()
            && &bytes[entry.offset..entry.offset + entry.magic.len()] == entry.magic
        {
            return Some(FileInfo {
                description: entry.description.to_string(),
            });
        }
    }
    // Fallback to infer if no custom magic matched
    if let Some(kind) = infer::get(bytes) {
        return Some(FileInfo {
            description: format!("{} ({})", kind.mime_type(), kind.matcher_type()),
        });
    }
    None
}
