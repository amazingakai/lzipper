// SPDX-License-Identifier: GPL-2.0-or-later
// SPDX-FileCopyrightText: 2025 Azhar Momin <azharmomin@proton.me>

use lzipper::{CompressionLevel, Decoder, Encoder};

use std::{
    fs::File,
    io::{Cursor, Read, Write},
    time::{SystemTime, UNIX_EPOCH},
};

#[test]
fn roundtrip() {
    let input = b"the quick brown fox jumps over the lazy dog";

    let mut encoded: Vec<u8> = Vec::new();
    let mut encoder = Encoder::new(input.as_slice()).expect("failed to setup encoder");
    encoder.encode(&mut encoded).expect("failed to encode");

    let mut decoded: Vec<u8> = Vec::new();
    let mut decoder = Decoder::new(Cursor::new(encoded));
    decoder.decode(&mut decoded).expect("failed to decode");

    assert_eq!(input, decoded.as_slice());
}

#[test]
fn roundtrip_files() {
    let input_data = b"hello world, this is a roundtrip file test!";

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let temp_dir = std::env::temp_dir();

    let input_path = temp_dir.join(format!("test_input_{}.txt", timestamp));
    let encoded_path = temp_dir.join(format!("test_encoded_{}.lz", timestamp));
    let decoded_path = temp_dir.join(format!("test_decoded_{}.txt", timestamp));

    // write input
    {
        let mut input_file = File::create(&input_path).expect("failed to create input file");
        input_file
            .write_all(input_data)
            .expect("failed to write input data");
    }

    // encode
    {
        let mut input_file =
            File::open(&input_path).expect("failed to open input file for reading");
        let mut encoded_file = File::create(&encoded_path).expect("failed to create encoded file");
        let mut encoder = Encoder::new(&mut input_file).expect("failed to setup encoder");
        encoder.encode(&mut encoded_file).expect("failed to encode");
    }

    // decode
    {
        let mut encoded_file = File::open(&encoded_path).expect("failed to open encoded file");
        let mut decoded_file = File::create(&decoded_path).expect("failed to create decoded file");
        let mut decoder = Decoder::new(&mut encoded_file);
        decoder.decode(&mut decoded_file).expect("failed to decode");
    }

    // verify
    {
        let mut decoded_file = File::open(&decoded_path).expect("failed to open decoded file");
        let mut decoded_data = Vec::new();
        decoded_file
            .read_to_end(&mut decoded_data)
            .expect("failed to read decoded data");

        assert_eq!(input_data.as_slice(), decoded_data.as_slice());
    }
}

#[test]
fn roundtrip_large() {
    let input = vec![0; 10 * 1024 * 1024]; // 10 MiB of zeros

    let mut encoded: Vec<u8> = Vec::new();
    let mut encoder = Encoder::new(input.as_slice()).expect("failed to setup encoder");
    encoder.encode(&mut encoded).expect("failed to encode");

    let mut decoded: Vec<u8> = Vec::new();
    let mut decoder = Decoder::new(Cursor::new(encoded));
    decoder.decode(&mut decoded).expect("failed to decode");

    assert_eq!(input, decoded.as_slice());
}
#[test]
fn roundtrip_max_compression_level() {
    let input = vec![0; 10 * 1024 * 1024]; // 10 MiB of zeros

    let mut encoded: Vec<u8> = Vec::new();
    let mut encoder = Encoder::new_with_level(input.as_slice(), CompressionLevel::Maximum)
        .expect("failed to setup encoder");
    encoder.encode(&mut encoded).expect("failed to encode");

    let mut decoded: Vec<u8> = Vec::new();
    let mut decoder = Decoder::new(Cursor::new(encoded));
    decoder.decode(&mut decoded).expect("failed to decode");

    assert_eq!(input, decoded.as_slice());
}
