use std::path::PathBuf;
use walkdir::WalkDir;

/// Gets a list of note files in the notes directory
pub fn get_note_files(notes_dir: &PathBuf) -> Vec<PathBuf> {
    let mut files = Vec::new();
    
    if !notes_dir.exists() {
        return files;
    }

    for entry in WalkDir::new(notes_dir)
        .min_depth(1)
        .max_depth(1)
        .sort_by_file_name()
    {
        if let Ok(entry) = entry {
            if entry.file_type().is_file() {
                files.push(entry.path().to_path_buf());
            }
        }
    }
    
    files
}
