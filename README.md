# Zed Encrypted Notes Plugin

A Zed editor extension for taking encrypted notes that automatically sync to a Git repository.

## Current Status

⚠️ **Note**: This extension is currently limited by Zed's extension API capabilities. While the core functionality (encryption, Git sync) is implemented, Zed's current extension system doesn't yet support the interactive commands needed for a full note-taking experience.

## Features (Planned)

- ✅ Encrypt notes using age encryption
- ✅ Automatic Git sync on save
- ✅ List available notes
- ❌ Interactive commands in Zed (waiting for API support)
- ❌ Note editor integration (waiting for API support)

## Installation

### For Development

1. Make sure you have Rust installed via [rustup](https://rustup.rs/)
2. Clone this repository
3. In Zed, go to Extensions > Install Dev Extension
4. Select this directory

### Building

```bash
cargo build --release
```

## Project Structure

```
zed_notes_plugin/
├── extension.toml      # Extension metadata
├── Cargo.toml         # Rust dependencies
└── src/
    ├── lib.rs         # Main extension entry point
    ├── crypto.rs      # Encryption/decryption logic
    ├── git.rs         # Git sync functionality
    └── ui.rs          # Note listing utilities
```

## Setup

1. **Create the Notes Directory:**
   The plugin automatically creates `~/.zed_encrypted_notes` on first run.

2. **Initialize Git Remote:**
   Navigate to the directory and set up your Git repository.

   ```sh
   cd ~/.zed_encrypted_notes
   git remote add origin <your-private-git-repo-url>
   ```

## How It Would Work (Once API Supports It)

1. **Save encrypted note**: `/note-save <name> <passphrase> <content>`
2. **Load encrypted note**: `/note-load <name> <passphrase>`
3. **List notes**: `/note-list`

Notes are stored in `~/.zed_encrypted_notes/` as `.enc` files.

## Technical Details

- Uses `age` for encryption
- Automatically initializes a Git repository in the notes directory
- Commits and pushes changes after each save

## Limitations

The current Zed extension API is primarily designed for:
- Language support (syntax highlighting, LSP)
- Themes and icons
- Basic slash commands (through a different mechanism)

For a fully interactive note-taking experience, we need:
- Custom UI panels
- File system watchers
- Interactive prompts for passwords
- Custom commands with better argument handling

## Alternative Approaches

Until Zed's extension API expands, you could:

1. Use this as a library and create a separate CLI tool
2. Create shell scripts that interact with the notes directory
3. Use Zed's built-in terminal to run encryption/decryption commands

## Contributing

Feel free to contribute! The core functionality is implemented and tested, we're just waiting for Zed's API to catch up.
