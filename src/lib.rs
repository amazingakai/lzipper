// SPDX-License-Identifier: GPL-2.0-or-later
// SPDX-FileCopyrightText: 2025 Azhar Momin <azharmomin@proton.me>

//! lzipper - A Rust library for for handling compression and decompression of lzip format.
//!
//! This crate provides `Encoder` and `Decoder` types for compressing + decompressing data
//! using the lzip format.
//!
//! # Example
//!
//! ```rust
//! use lzipper::{Decoder, Encoder};
//! use std::io::Cursor;
//!
//! // A basic roundtrip example
//! let input = b"the quick brown fox jumps over the lazy dog";
//!
//! let mut encoded = Vec::new();
//! let mut encoder = Encoder::new(input.as_slice());
//! encoder.encode(&mut encoded).expect("failed to encode");
//!
//! let mut decoded = Vec::new();
//! let mut decoder = Decoder::new(Cursor::new(encoded));
//! decoder.decode(&mut decoded).expect("failed to decode");
//!
//! assert_eq!(input, decoded.as_slice());
//! ```

#![deny(missing_docs)]

pub mod decoder;
pub mod encoder;
pub mod error;

pub use crate::error::LzipError;

pub use crate::decoder::Decoder;
pub use crate::encoder::{CompressionLevel, Encoder};

pub(crate) const MIN_DICT_SIZE: u32 = 1 << 12; // 4 KiB
pub(crate) const MAX_DICT_SIZE: u32 = 1 << 29; // 512 MiB

pub(crate) const LZIP_MAGIC: [u8; 4] = [0x4C, 0x5A, 0x49, 0x50];
pub(crate) const LZIP_VERSION: u8 = 0x01;

const LZMA_PRESET_DEFAULT: u32 = 6;
