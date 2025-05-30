use std::fs::{self, File};
use std::io::{self, Write, Read};
use std::path::PathBuf;
use dirs::home_dir;

use aes_gcm::aead::{Aead, KeyInit, OsRng, generic_array::GenericArray};
use aes_gcm::{Aes256Gcm, Nonce}; // 96-bit nonce
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;
use rand::RngCore;

use crate::editor::start_editor_with_content; // add this to top if needed


const SALT: &[u8] = b"geekpad-fixed-salt"; // You can improve by generating per note


pub fn create_note(name: &str) {
    let path = get_note_path(name);
    if path.exists() {
        println!("⚠️ Note already exists.");
        return;
    }

    println!("🔐 Enter password:");
    let password = rpassword::prompt_password(" > ").unwrap();

    println!("📝 Start typing your note. Type `/alldone` on a new line to finish:\n");

    let mut note_lines = Vec::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        if io::stdin().read_line(&mut line).is_err() {
            println!("⚠️ Failed to read line.");
            continue;
        }

        let trimmed = line.trim_end();

        if trimmed == "/alldone" {
            break;
        }

        note_lines.push(trimmed.to_string());
    }

    let note = note_lines.join("\n");


    let encrypted = encrypt_note(&note, &password);
    fs::write(path, encrypted).expect("Failed to save note");

    println!("✅ Note saved securely.");
}

pub fn view_note(name: &str) {
    let path = get_note_path(name);
    if !path.exists() {
        println!("❌ Note not found.");
        return;
    }

    println!("🔐 Enter password:");
    let password = rpassword::prompt_password(" > ").unwrap();

    let data = fs::read(path).expect("Failed to read file");
    match decrypt_note(&data, &password) {
        Ok(decrypted) => {
            println!("\n🔓 Decrypted Note:");
            println!("─────────────────────────────");
            println!("{}", decrypted);
        }
        Err(_) => println!("❌ Incorrect password or corrupted file."),
    }
}

pub fn delete_note(name: &str) {
    let path = get_note_path(name);
    if path.exists() {
        fs::remove_file(path).expect("Failed to delete note");
        println!("🗑️ Note deleted.");
    } else {
        println!("❌ Note not found.");
    }
}

pub fn list_notes() {
    let notes_dir = get_notes_dir();

    let entries = match fs::read_dir(&notes_dir) {
        Ok(dir) => dir,
        Err(_) => {
            println!("⚠️ Could not read notes directory.");
            return;
        }
    };

    println!("📄 Your Notes:");
    for entry in entries {
        if let Ok(entry) = entry {
            if let Some(name) = entry.file_name().to_str() {
                println!("• {}", name);
            }
        }
    }
}


pub fn change_theme() {
    println!("🎨 Theme changing not implemented yet. Coming soon!");
}

fn get_notes_dir() -> PathBuf {
    let mut dir = home_dir().unwrap_or_else(|| PathBuf::from("."));
    dir.push(".geekpad_notes");
    fs::create_dir_all(&dir).expect("Could not create notes directory");
    dir
}

fn get_note_path(name: &str) -> PathBuf {
    let mut path = get_notes_dir();
    path.push(format!("{}.enc", name));
    path
}

pub fn edit_note(name: &str) {
    let path = get_note_path(name);
    if !path.exists() {
        println!("❌ Note not found.");
        return;
    }

    println!("🔐 Enter password:");
    let password = rpassword::prompt_password(" > ").unwrap();

    let data = fs::read(&path).expect("Failed to read file");
    let decrypted = match decrypt_note(&data, &password) {
        Ok(text) => text,
        Err(_) => {
            println!("❌ Incorrect password or corrupted file.");
            return;
        }
    };

    // Launch the editor with the existing content
    match start_editor_with_content(&decrypted) {
        Ok(edited_content) => {
            let encrypted = encrypt_note(&edited_content, &password);
            fs::write(&path, encrypted).expect("Failed to save edited note");
            println!("✅ Note updated successfully.");
        }
        Err(e) => {
            println!("⚠️ Editor error: {}", e);
        }
    }
}

// Placeholder encryption/decryption (to be replaced with real AES later)
fn encrypt_note(note: &str, password: &str) -> Vec<u8> {
    // Derive key from password
    let mut key = [0u8; 32];
    pbkdf2_hmac::<Sha256>(password.as_bytes(), SALT, 100_000, &mut key);
    let key = GenericArray::from_slice(&key);
    let cipher = Aes256Gcm::new(key);

    // Generate nonce
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // Encrypt
    let ciphertext = cipher.encrypt(nonce, note.as_bytes()).expect("encryption failure!");

    // Return nonce + ciphertext
    [nonce_bytes.to_vec(), ciphertext].concat()
}

fn decrypt_note(data: &[u8], password: &str) -> Result<String, ()> {
    if data.len() < 12 {
        return Err(()); // Invalid data
    }

    let (nonce_bytes, ciphertext) = data.split_at(12);

    // Derive key from password
    let mut key = [0u8; 32];
    pbkdf2_hmac::<Sha256>(password.as_bytes(), SALT, 100_000, &mut key);
    let key = GenericArray::from_slice(&key);
    let cipher = Aes256Gcm::new(key);

    let nonce = Nonce::from_slice(nonce_bytes);
    let plaintext = cipher.decrypt(nonce, ciphertext).map_err(|_| ())?;
    let text = String::from_utf8(plaintext).map_err(|_| ())?;

    Ok(text)
}

