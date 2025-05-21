// SPDX-License-Identifier: GPL-2.0-or-later
// SPDX-FileCopyrightText: 2025 Azhar Momin <azharmomin@proton.me>

use lzipper::{Decoder, LzipError};

#[test]
fn invalid_magic() {
    let corrupt_data = b"this does not start with LZIP magic";

    let mut decoder = Decoder::new(corrupt_data.as_slice());

    let result = decoder.decode(&mut Vec::new());

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), LzipError::InvalidMagic);
}

#[test]
fn invalid_version() {
    let corrupt_data = b"LZIP\x00\x00\x00\x00";

    let mut decoder = Decoder::new(corrupt_data.as_slice());
    let result = decoder.decode(&mut Vec::new());

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), LzipError::UnsupportedVersion);
}

#[test]
fn invalid_dict_size() {
    let corrupt_data = b"LZIP\x01\x00\x00\x00\x00\x00\x00\x00";

    let mut decoder = Decoder::new(corrupt_data.as_slice());
    let result = decoder.decode(&mut Vec::new());

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), LzipError::InvalidDictSize);
}

#[test]
fn invalid_data() {
    let corrupt_data = b"LZIP\x01\x0c\x00\x00\x00\x00\x00\x00";

    let mut decoder = Decoder::new(corrupt_data.as_slice());
    let result = decoder.decode(&mut Vec::new());

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), LzipError::UnexpectedEndOfStream);
}

#[test]
fn invalid_crc() {
    let corrupt_data = b"LZIP\x01\x0c\x00\x34\x19\x49\xee\x8d\xdd\x3d\x3a\xdf\xff\xff\xdd\x12\x00\x00\x00\x00\x00\x00\x06\x00\x00\x00\x00\x00\x00\x00\x2a\x00\x00\x00\x00\x00\x00\x00";

    let mut decoder = Decoder::new(corrupt_data.as_slice());
    let result = decoder.decode(&mut Vec::new());

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), LzipError::InvalidCrc);
}

#[test]
fn invalid_data_size() {
    let corrupt_data = b"LZIP\x01\x0c\x00\x34\x19\x49\xee\x8d\xdd\x3d\x3a\xdf\xff\xff\xdd\x12\x00\x00\x20\x30\x3a\x36\x00\x00\x00\x00\x00\x00\x00\x00\x2a\x00\x00\x00\x00\x00\x00\x00";

    let mut decoder = Decoder::new(corrupt_data.as_slice());
    let result = decoder.decode(&mut Vec::new());

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), LzipError::InvalidDataSize);
}

#[test]
fn invalid_member_size() {
    let corrupt_data = b"LZIP\x01\x0c\x00\x34\x19\x49\xee\x8d\xdd\x3d\x3a\xdf\xff\xff\xdd\x12\x00\x00\x20\x30\x3a\x36\x06\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";

    let mut decoder = Decoder::new(corrupt_data.as_slice());
    let result = decoder.decode(&mut Vec::new());

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), LzipError::InvalidMemberSize);
}
