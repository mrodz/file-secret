use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

use anyhow::anyhow;

use chacha20poly1305::{
    aead::{stream, NewAead},
    XChaCha20Poly1305,
};

use indicatif::{ProgressBar, ProgressStyle};

pub fn file_exists(path: &String) -> bool {
    Path::new(path).exists()
}

pub fn yield_file_path(path: &String) -> String {
    fn init(path: &String, n: u8) -> String {
        if file_exists(path) {
            let mut i = 0;
            for c in path.chars().rev() {
                i += 1;

                if c == '.' {
                    break;
                }
            }

            let without_extension = &path[..path.len() - i];
            let extension = &path[path.len() - i..];

            let new_path = format!("{without_extension} ({n}){extension}");

            return if file_exists(&new_path) {
                init(&path, n + 1)
            } else {
                new_path
            };
        }

        path.to_string()
    }

    init(path, 1)
}

pub fn encrypt_large_file(
    source_file_path: &str,
    key: &[u8; 32],
    nonce: &[u8; 19],
) -> std::result::Result<(), anyhow::Error> {
    let aead = XChaCha20Poly1305::new(key.as_ref().into());
    let mut stream_encryptor = stream::EncryptorBE32::from_aead(aead, nonce.as_ref().into());

    const BUFFER_LEN: usize = 500;
    let mut buffer = [0u8; BUFFER_LEN];

    let mut source_file = File::open(source_file_path)?;
    let mut dist_file = File::create(yield_file_path(
        &(source_file_path.to_owned() + ".LOCKED").to_string(),
    ))?;

    let mut hashed: u64 = 0;
    let total_size = source_file.metadata()?.len();

    println!("Locking file...");
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{bar:.green/.yellow}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        // .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("=>-"));

    loop {
        let read_count = source_file.read(&mut buffer)?;

        let new = std::cmp::min(hashed + BUFFER_LEN as u64, total_size);

        hashed = new;

        pb.set_position(new);

        if read_count == BUFFER_LEN {
            let ciphertext = stream_encryptor
                .encrypt_next(buffer.as_slice())
                .map_err(|err| anyhow!("Encrypting large file: {}", err))?;
            dist_file.write(&ciphertext)?;
        } else {
            let ciphertext = stream_encryptor
                .encrypt_last(&buffer[..read_count])
                .map_err(|err| anyhow!("Encrypting large file: {}", err))?;
            dist_file.write(&ciphertext)?;
            break;
        }
    }

    pb.finish_and_clear();

    Ok(())
}

pub fn decrypt_large_file(
    encrypted_file_path: &str,
    key: &[u8; 32],
    nonce: &[u8; 19],
) -> std::result::Result<(), anyhow::Error> {
    assert!(encrypted_file_path.ends_with(".LOCKED"));

    let aead = XChaCha20Poly1305::new(key.as_ref().into());
    let mut stream_decryptor = stream::DecryptorBE32::from_aead(aead, nonce.as_ref().into());

    const BUFFER_LEN: usize = 500 + 16;
    let mut buffer = [0u8; BUFFER_LEN];

    let mut encrypted_file = File::open(encrypted_file_path)?;

    let mut destination = encrypted_file_path[..encrypted_file_path.len() - 7].to_string();

    if let Some(')') = destination.chars().next_back() {
        destination.pop();

        loop {
            if let Some(c) = destination.chars().next_back() {
                if c.is_numeric() {
                    destination.pop();
                    continue;
                }
            }
            break;
        }

        destination.pop();
        destination.pop();
    }

    let mut dist_file = File::create(yield_file_path(&destination))?;

    let mut hashed: u64 = 0;
    let total_size = encrypted_file.metadata()?.len();

    println!("Unlocking file...");

    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{bar:.green/.yellow}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .progress_chars("=>-"));

    loop {
        let read_count = encrypted_file.read(&mut buffer)?;

        let new = std::cmp::min(hashed + BUFFER_LEN as u64, total_size);

        hashed = new;

        pb.set_position(new);

        if read_count == BUFFER_LEN {
            let plaintext = stream_decryptor
                .decrypt_next(buffer.as_slice())
                .map_err(|err| anyhow!("Decrypting large file ({encrypted_file_path}): {}", err))?;
            dist_file.write(&plaintext)?;
        } else if read_count == 0 {
            break;
        } else {
            let plaintext = stream_decryptor
                .decrypt_last(&buffer[..read_count])
                .map_err(|err| anyhow!("Decrypting large file ({encrypted_file_path}): {}", err))?;
            dist_file.write(&plaintext)?;
            break;
        }
    }

    pb.finish_and_clear();

    Ok(())
}