# ufile-core

A high-performance, platform-agnostic Rust library for file type identification based on magic numbers and binary signatures. This is the core library that powers various file identification frontends.

## Overview

`ufile-core` provides reliable file type detection through:

- **Magic number analysis**: Custom database of file signatures for accurate detection
- **Multi-file processing**: Efficiently process multiple files and directories
- **Fallback detection**: Integration with the `infer` crate for additional coverage
- **Cross-platform**: Works on all platforms supported by Rust

This library is designed to be integrated into CLI tools, web applications (via WASM), desktop applications, and other software requiring file type identification.

## Where I've Used It

`ufile-core` forms the foundational logic for my file identification projects:

  * **`ufile-cli`**: I've developed a native command-line interface (CLI) tool that directly utilizes `ufile-core` to identify files on your local system. You can find this project on GitHub: [ufile-cli](https://github.com/rsomonte/ufile-cli).
  * **`ufile-wasm`**: This WebAssembly module brings `ufile-core`'s capabilities to the web. I've integrated it into my personal website, `rsomonte.github.io`, to enable client-side file type detection directly in the browser. The source for this integration is housed within my website's repository: [rsomonte.github.io](https://github.com/rsomonte/rsomonte.github.io).

Both the CLI and Wasm frontends integrate `ufile-core` as a direct Git dependency, ensuring they always use the exact same, version-controlled identification logic.

<img width="3840" height="2233" alt="Untitled diagram _ Mermaid Chart-2025-07-30-011157" src="https://github.com/user-attachments/assets/f01e13cc-7a9f-464b-b13f-322164022cac" />

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
ufile-core = { git = "https://github.com/rsomonte/ufile-core.git", branch = "main" }
```


## Data Types

### FileInfo

The main result type containing file information:

```rust
pub struct FileInfo {
    /// The path to the file
    pub path: PathBuf,
    /// Human-readable description of the file type
    pub description: String,
    /// Whether this entry represents a directory
    pub is_directory: bool,
    /// File size in bytes (None for directories)
    pub size: Option<u64>,
}
```

### FileProcessingError

Comprehensive error handling for file operations:

```rust
pub enum FileProcessingError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Path does not exist: {0}")]
    PathNotFound(PathBuf),
    #[error("Directory traversal error: {0}")]
    WalkDir(#[from] walkdir::Error),
}
```


## Performance Considerations

- **Memory efficient**: Processes files without loading entire contents into memory when possible
- **Lazy evaluation**: Only reads file data when necessary for identification
- **Batch processing**: Efficiently handles multiple files with minimal system calls
- **Early detection**: Magic number matching stops at first successful identification

## Simple Usage

```rust
use ufile_core::*;

// Identify a single file from bytes
let bytes = std::fs::read("myfile.png")?;
let info = identify_from_bytes(&bytes);
println!("Type: {}", info.map_or("Unknown", |i| &i.description));

// Identify multiple files from bytes
let files: Vec<Vec<u8>> = vec![ /* ...file contents... */ ];
let results = identify_many_bytes(files.iter().map(|v| v.as_slice()));
for info in results {
    println!("Type: {}", info.description);
}
```


