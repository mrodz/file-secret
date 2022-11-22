use std::{
    fs::File,
    io::{Read, Write},
	path::Path
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

pub fn encrypt_large_file(
    source_file_path: &str,
    dist_file_path: &str,
    key: &[u8; 32],
    nonce: &[u8; 19],
) -> std::result::Result<(), anyhow::Error> {
    let aead = XChaCha20Poly1305::new(key.as_ref().into());
    let mut stream_encryptor = stream::EncryptorBE32::from_aead(aead, nonce.as_ref().into());

    const BUFFER_LEN: usize = 500;
    let mut buffer = [0u8; BUFFER_LEN];

    let mut source_file = File::open(source_file_path)?;
    let mut dist_file = File::create(dist_file_path)?;

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

    // pb.finish();
    pb.finish_and_clear();

    Ok(())
}

pub fn decrypt_large_file(
    encrypted_file_path: &str,
    dist: &str,
    key: &[u8; 32],
    nonce: &[u8; 19],
) -> std::result::Result<(), anyhow::Error> {
    let aead = XChaCha20Poly1305::new(key.as_ref().into());
    let mut stream_decryptor = stream::DecryptorBE32::from_aead(aead, nonce.as_ref().into());

    const BUFFER_LEN: usize = 500 + 16;
    let mut buffer = [0u8; BUFFER_LEN];

    let mut encrypted_file = File::open(encrypted_file_path)?;
    let mut dist_file = File::create(dist)?;

    let mut hashed: u64 = 0;
    let total_size = encrypted_file.metadata()?.len();

    println!("Unlocking file...");

    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{bar:.green/.yellow}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        // .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("=>-"));

    loop {
        let read_count = encrypted_file.read(&mut buffer)?;

        let new = std::cmp::min(hashed + BUFFER_LEN as u64, total_size);

        hashed = new;

        pb.set_position(new);

        if read_count == BUFFER_LEN {
            let plaintext = stream_decryptor
                .decrypt_next(buffer.as_slice())
                .map_err(|err| { anyhow!("Decrypting large file ({encrypted_file_path}): {}", err) })?;
            dist_file.write(&plaintext)?;
        } else if read_count == 0 {
            break;
        } else {
            let plaintext = stream_decryptor
                .decrypt_last(&buffer[..read_count])
                .map_err(|err| { anyhow!("Decrypting large file ({encrypted_file_path}): {}", err) })?;
            dist_file.write(&plaintext)?;
            break;
        }
    }

    // pb.finish();
    pb.finish_and_clear();

    Ok(())
}

// pub fn read_file(path: &String) -> Result<String> {
//     let f = match File::open(path) {
//         Ok(o) => o,
//         Err(e) => {
//             return Err(Box::new(FileError {
//                 file_name: path.to_owned(),
//                 message: e.kind().to_string(),
//             }))
//         }
//     };

//     let mut data = String::new();

//     let br = BufReader::new(f).read_to_string(&mut data);

//     if br.is_err() {
//         Err(Box::new(FileError {
//             file_name: path.to_string(),
//             message: String::from("could not read"),
//         }))
//     } else {
//         Ok(data)
//     }
// }
