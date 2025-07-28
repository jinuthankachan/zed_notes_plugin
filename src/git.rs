use chrono::Local;
use std::process::Command;
use std::path::PathBuf;

/// Executes git add, commit, and push for a file.
pub fn sync_to_remote(file_path: PathBuf, notes_dir: PathBuf) -> Result<String, String> {
    let default_message = Local::now().format("Update: %Y-%m-%d %H:%M:%S").to_string();
    
    // 1. Git Add
    if !Command::new("git")
        .current_dir(&notes_dir)
        .arg("add")
        .arg(&file_path)
        .status()
        .map_or(false, |s| s.success())
    {
        return Err("Git: Failed to stage file.".to_string());
    }

    // 2. Git Commit
    if !Command::new("git")
        .current_dir(&notes_dir)
        .arg("commit")
        .arg("-m")
        .arg(&default_message)
        .status()
        .map_or(false, |s| s.success())
    {
        return Ok("Git: Nothing to commit.".to_string());
    }

    // 3. Git Push
    let push_output = Command::new("git")
        .current_dir(&notes_dir)
        .arg("push")
        .output();
    match push_output {
        Ok(output) if output.status.success() => {
            Ok("✅ Note synced to remote.".to_string())
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(format!("Git Push Failed: {}", stderr))
        }
        Err(e) => {
            Err(format!("Git Push command failed: {}", e))
        }
    }
}
