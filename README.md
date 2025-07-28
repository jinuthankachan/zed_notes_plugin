# Zed Encrypted Notes Plugin

A simple note-taking plugin for the Zed editor that encrypts notes on disk and automatically syncs them to a remote Git repository on save.

## Setup

1.  **Create the Notes Directory:**
    The plugin uses `~/.zed_encrypted_notes`.

2.  **Initialize Git:**
    Navigate to the directory and set up your Git repository.

    ```sh
    cd ~/.zed_encrypted_notes
    git init
    git remote add origin <your-private-git-repo-url>
    ```

## Usage

-   Click on a note in the "Notes" panel to open and decrypt it.
-   Save the note (Cmd+S) to encrypt the content, save it to disk, and push it to your remote repository.
