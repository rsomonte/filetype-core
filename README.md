# ufile-core: Universal File Type Identification

`ufile-core` is a high-performance, platform-agnostic Rust library designed to accurately identify file types based on their unique binary signatures, known as "magic numbers". It provides a robust and centralized solution for applications requiring reliable file detection. This project draws inspiration from the classic Unix `file` command.

## Where I've Used It

`ufile-core` forms the foundational logic for my file identification projects:

  * **`ufile-cli`**: I've developed a native command-line interface (CLI) tool that directly utilizes `ufile-core` to identify files on your local system. You can find this project on GitHub: [ufile-cli](https://github.com/rsomonte/ufile-cli).
  * **`ufile-wasm`**: This WebAssembly module brings `ufile-core`'s capabilities to the web. I've integrated it into my personal website, `rsomonte.github.io`, to enable client-side file type detection directly in the browser. The source for this integration is housed within my website's repository: [rsomonte.github.io](https://github.com/rsomonte/rsomonte.github.io).

Both the CLI and Wasm frontends integrate `ufile-core` as a direct Git dependency, ensuring they always use the exact same, version-controlled identification logic.

<img width="3840" height="2233" alt="Untitled diagram _ Mermaid Chart-2025-07-30-011157" src="https://github.com/user-attachments/assets/f01e13cc-7a9f-464b-b13f-322164022cac" />

## Usage (Core Library)

To integrate `ufile-core` into your Rust project, add it as a dependency:

```toml
[dependencies]
ufile-core = { git = "https://github.com/rsomonte/ufile-core.git", branch = "main" }
```

Then, you can use the `identify_from_bytes` function:

```rust
pub fn identify_from_bytes(bytes: &[u8]) -> Option<FileInfo> { /* ... */ }
```

The `FileInfo` struct provides a human-readable `description` of the detected file type.

`ufile-core` empowers developers to easily integrate powerful and consistent file type identification across various platforms and applications.
