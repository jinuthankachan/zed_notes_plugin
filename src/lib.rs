mod crypto;
mod git;
mod ui;

use once_cell::sync::Lazy;
use std::path::PathBuf;
use zed_extension_api::{self as zed};

// A global, lazily-initialized path for the notes directory.
static NOTES_DIR: Lazy<PathBuf> = Lazy::new(|| {
    dirs::home_dir()
        .expect("Could not find home directory")
        .join(".zed_encrypted_notes")
});

struct NotesPlugin;

impl zed::Extension for NotesPlugin {
    fn new() -> Self {
        // Ensure the notes directory exists on startup.
        if !NOTES_DIR.exists() {
            std::fs::create_dir_all(NOTES_DIR.as_path()).expect("Could not create notes directory");
        }
        
        // Initialize git repo if not exists
        let git_dir = NOTES_DIR.join(".git");
        if !git_dir.exists() {
            std::process::Command::new("git")
                .current_dir(NOTES_DIR.as_path())
                .arg("init")
                .output()
                .ok();
        }
        
        Self
    }
    
    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command, String> {
        Err("NotesPlugin does not provide language servers".into())
    }
}

zed::register_extension!(NotesPlugin);
