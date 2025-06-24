# filetype-core: Universal File Type Identification

`filetype-core` is a high-performance, platform-agnostic Rust library designed to accurately identify file types based on their unique binary signatures, known as "magic numbers". It provides a robust and centralized solution for applications requiring reliable file detection. This project draws inspiration from the classic Unix `file` command.

## Where I've Used It

`filetype-core` forms the foundational logic for my file identification projects:

  * **`filetype-cli`**: I've developed a native command-line interface (CLI) tool that directly utilizes `filetype-core` to identify files on your local system. You can find this project on GitHub: [filetype-cli](https://github.com/rsomonte/filetype-cli).
  * **`filetype-wasm`**: This WebAssembly module brings `filetype-core`'s capabilities to the web. I've integrated it into my personal website, `rsomonte.github.io`, to enable client-side file type detection directly in the browser. The source for this integration is housed within my website's repository: [rsomonte.github.io](https://github.com/rsomonte/rsomonte.github.io).

Both the CLI and Wasm frontends integrate `filetype-core` as a direct Git dependency, ensuring they always use the exact same, version-controlled identification logic.

![Editor _ Mermaid Chart-2025-06-24-022454](https://github.com/user-attachments/assets/06bfeb27-a110-4631-bbaa-7f042f2e3ae3)

## Usage (Core Library)

To integrate `filetype-core` into your Rust project, add it as a dependency:

```toml
[dependencies]
filetype-core = { git = "https://github.com/rsomonte/filetype-core.git", branch = "main" }
```

Then, you can use the `identify_from_bytes` function:

```rust
pub fn identify_from_bytes(bytes: &[u8]) -> Option<FileInfo> { /* ... */ }
```

The `FileInfo` struct provides a human-readable `description` of the detected file type.

`filetype-core` empowers developers to easily integrate powerful and consistent file type identification across various platforms and applications.
