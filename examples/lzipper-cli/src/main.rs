// SPDX-License-Identifier: GPL-2.0-or-later
// SPDX-FileCopyrightText: 2025 Azhar Momin <azharmomin@proton.me>

use lzipper::{Decoder, Encoder, EncoderOptions};
use std::path::Path;
use std::{env, fs::File, io};

struct Args {
    mode: String,
    file_path: String,
}

impl Args {
    fn parse() -> Self {
        let args: Vec<String> = env::args().collect();

        let mut mode = None;
        let mut file_path = None;

        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "--mode" => {
                    if i + 1 < args.len() {
                        mode = Some(args[i + 1].clone());
                        i += 1;
                    }
                }
                _ => {
                    if file_path.is_none() {
                        file_path = Some(args[i].clone());
                    }
                }
            }
            i += 1;
        }

        if mode.is_none() {
            eprintln!("Error: --mode is required.");
            std::process::exit(1);
        }

        if file_path.is_none() {
            eprintln!("Error: File path is required.");
            std::process::exit(1);
        }

        Args {
            mode: mode.unwrap(),
            file_path: file_path.unwrap(),
        }
    }

    fn validate_file(&self) -> Result<(), &'static str> {
        let path = Path::new(&self.file_path);
        if !path.exists() || !path.is_file() {
            return Err("Error: Invalid file or file does not exist.");
        }

        Ok(())
    }
}

fn compress_file(file_path: &str) -> io::Result<()> {
    let output_file_path = format!("{}.lz", file_path);

    let file = File::open(file_path)?;
    let mut output_file = File::create(&output_file_path)?;

    let mut encoder = Encoder::new(EncoderOptions::default(), file)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    encoder
        .encode(&mut output_file)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    println!("File written to: {}", &output_file_path);
    Ok(())
}

fn decompress_file(file_path: &str) -> io::Result<()> {
    let output_file_path = file_path.trim_end_matches(".lz").to_string();

    let file = File::open(file_path)?;
    let mut output_file = File::create(&output_file_path)?;

    let mut decoder = Decoder::new(file);
    decoder
        .decode(&mut output_file)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    println!("File written to: {}", &output_file_path);
    Ok(())
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    if let Err(e) = args.validate_file() {
        eprintln!("{}", e);
        return Ok(());
    }

    match args.mode.as_str() {
        "compress" => compress_file(&args.file_path)?,
        "decompress" => decompress_file(&args.file_path)?,
        _ => eprintln!("Error: Invalid mode. Use 'compress' or 'decompress'."),
    }

    Ok(())
}
