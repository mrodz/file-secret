mod errors;
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
    /// Name of the person to greet
    #[arg(required(true), index = 1)]
    path: String,
    #[arg(short = 'p')]
    password: Option<String>,
    #[arg(short = 'l', conflicts_with("unlock"))]
    lock: bool,
    #[arg(short = 'u', conflicts_with("lock"))]
    unlock: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let action: u8 = ((args.unlock as u8) << 1) | (args.lock as u8);
    let with_extension = args.path.to_owned() + ".LOCKED";

    let defacto_path = if action == 0b10 {
        with_extension.clone()
    } else {
        args.path.clone()
    };

    if !file_exists(&defacto_path) {
        let mut e = Command::new("path");
        e.error(ErrorKind::ValueValidation, "The file does not exist!")
            .exit();
    }

    // let file_content = files::read_file(&args.path).unwrap();

    // encrypt(&file_content, &args.password);

    // 0b10 if decrypt; 0b00 if encrypt.

    let as_fn = if action == 0b10 {
        decrypt_large_file
    } else {
        encrypt_large_file
    };

    let arguments: [&String; 2] = [&args.path, &with_extension];

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
    // OsRng.fill_bytes(&mut large_file_key);
    // OsRng.fill_bytes(&mut large_file_nonce);

    if as_fn(
        arguments[if action == 0b10 { 1 } else { 0 }],
        arguments[if action == 0b10 { 0 } else { 1 }],
        &large_file_key,
        &large_file_nonce,
    ).is_err() {
        remove_file(arguments[if action == 0b10 { 0 } else { 1 }])?;
        eprintln!("Incorrect password.");
        exit(1);
    }

    println!("Finishing...");
    let pb = ProgressBar::new(3);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{bar:.green/.yellow}] {pos}/{len}")
        .unwrap()
        // .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("=>-"));

    pb.set_position(1);
    remove_file(&defacto_path)?;
    pb.set_position(3);
    pb.finish_and_clear();
    println!("Done!");
    Ok(())
}
