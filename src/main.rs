mod files;

extern crate rpassword;

use std::{
    fs::{remove_file},
    process::exit,
};

use clap::{error::ErrorKind, Command, Parser};
use files::*;
use indicatif::{ProgressBar, ProgressStyle};
// use rand::{rngs::OsRng, RngCore};
// use std::io::Write;
// use rpassword::read_password;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Specify the path to the file
    #[arg(required(true), index = 1)]
    path: String,

    /// Enter a password to accompany your request. 
    /// WARNING: if locking a file, will not check input matches your intended password!
    #[arg(short = 'p')]
    password: Option<String>,

    /// Specify that you intend to lock the file. If neither  -l or -u are present, this is the default.
    #[arg(short = 'l', conflicts_with("unlock"))]
    lock: bool,

    /// Specify that you intend to unlock the file.
    #[arg(short = 'u', conflicts_with("lock"))]
    unlock: bool,

    /// Do not replace the original file with the locked, and keep both.
    #[arg(short = 'P', long = "preserve")]
    preserve_original: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let action: u8 = ((args.unlock as u8) << 1) | (args.lock as u8);

    if !file_exists(&args.path) {
        let mut e = Command::new("any file");
        e.error(ErrorKind::ValueValidation, "That file does not exist.")
            .exit();
    }

    if !args.path.ends_with(".LOCKED") && action == 0b10 {
        let mut e = Command::new("any file with a .LOCKED extension");
        e.error(ErrorKind::ValueValidation, "Attempting to unlock a file, found wrong file extension. Expected *.LOCKED")
            .exit();
    }

    // 0b10 if decrypt; 0b00 if encrypt.

    let as_fn = if action == 0b10 {
        decrypt_large_file
    } else {
        encrypt_large_file
    };

    let mut large_file_key = [0u8; 32];
    let mut large_file_nonce = [0u8; 19];

    let password = if let Some(password) = args.password {
        password
    } else {
        let password1 = rpassword::prompt_password("Enter a password: ").unwrap();

        if action == 0b10 {
            password1
        } else {
            let password2 = rpassword::prompt_password("Confirm password: ").unwrap();
            if password1 != password2 {
                eprintln!("Passwords do not match.");
                exit(1);
            }

            password1
        }
    };

    use std::cmp::min;
    large_file_key[..min(32, password.len())].clone_from_slice(password.as_bytes());
    large_file_nonce[..min(19, password.len())].clone_from_slice(password.as_bytes());

    if let Err(_) = as_fn(
        &args.path,
        &large_file_key,
        &large_file_nonce,
    ) {
        eprintln!("Incorrect password. The resulting file will be filled with garbage.");
        exit(1);
    }

    println!("Finishing...");
    let pb = ProgressBar::new(3);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{bar:.green/.yellow}] {pos}/{len}")
        .unwrap()
        .progress_chars("=>-"));

    pb.set_position(1);

    if !args.preserve_original {
        remove_file(&args.path)?;
        pb.set_position(2);
    }

    pb.finish_and_clear();
    println!("Done!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::files::yield_file_path;

    #[test]
    pub fn duplicate() {
        assert_eq!(yield_file_path(&"test/The Spot6=.png".to_string()), "test/The Spot6= (2).png")
    }
}