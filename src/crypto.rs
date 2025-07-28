use age::{Decryptor, Encryptor, secrecy::Secret};
use anyhow;
use std::io::{Read, Write};
use std::path::Path;

/// Encrypts plaintext content and saves it to a file.
pub fn encrypt_and_save(content: &str, path: &Path, passphrase: &str) -> Result<(), anyhow::Error> {
    let encryptor = Encryptor::with_user_passphrase(Secret::new(passphrase.to_string()));

    let mut encrypted_file = Vec::new();
    let mut writer = encryptor.wrap_output(&mut encrypted_file)?;
    writer.write_all(content.as_bytes())?;
    writer.finish()?;

    std::fs::write(path, encrypted_file)?;
    Ok(())
}

/// Reads an encrypted file and decrypts its content.
pub fn decrypt_file(path: &Path, passphrase: &str) -> Result<String, anyhow::Error> {
    let encrypted_data = std::fs::read(path)?;
    let decryptor = match Decryptor::new(&encrypted_data[..])? {
        Decryptor::Passphrase(d) => d,
        _ => return Err(anyhow::anyhow!("File is not passphrase-encrypted")),
    };

    let secret_passphrase = Secret::new(passphrase.to_string());
    let mut decrypted = Vec::new();
    let mut reader = decryptor.decrypt(&secret_passphrase, None)?;
    reader.read_to_end(&mut decrypted)?;

    Ok(String::from_utf8(decrypted)?)
}
