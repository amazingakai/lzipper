// SPDX-License-Identifier: GPL-2.0-or-later
// SPDX-FileCopyrightText: 2025 Azhar Momin <azharmomin@proton.me>

//! The error module for the lzipper crate.

use std::{error, fmt, io};

use liblzma::stream;

#[derive(Debug, PartialEq)]
/// An error type for the lzipper crate.
pub enum LzipError {
    /// An error indicating that the magic number is invalid.
    InvalidMagic,
    /// An error indicating that the lzip version is invalid or unsupported.
    UnsupportedVersion,
    /// An error indicating that the dictionary size is invalid.
    /// The dictionary size must be between 4 KiB and 512 MiB.
    InvalidDictSize,
    /// An error indicating that the stream ended unexpectedly.
    UnexpectedEndOfStream,
    /// An error indicating that the CRC32 checksum is invalid.
    InvalidCrc,
    /// An error indicating that the size of the uncompressed data is invalid.
    InvalidDataSize,
    /// An error indicating that the size of the member is invalid.
    InvalidMemberSize,
    /// An error indicating that the LZMA stream encountered an error.
    StreamError(stream::Error),
    /// An error indicating that an I/O operation failed.
    /// This error wraps the underlying `io::Error`.
    /// This can occur during reading or writing operations.
    IoError(io::ErrorKind),
}

impl error::Error for LzipError {}

impl fmt::Display for LzipError {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LzipError::InvalidMagic => write!(f, "invalid magic number"),
            LzipError::UnsupportedVersion => write!(f, "unsupported lzip version"),
            LzipError::InvalidDictSize => write!(f, "invalid dictionary size (must be between 4 KiB and 512 MiB)"),
            LzipError::UnexpectedEndOfStream => write!(f, "unexpected end of stream"),
            LzipError::InvalidCrc => write!(f, "invalid CRC32 checksum"),
            LzipError::InvalidDataSize => write!(f, "invalid size of uncompressed data"),
            LzipError::InvalidMemberSize => write!(f, "invalid size of member"),
            LzipError::StreamError(err) => write!(f, "{}", err),
            LzipError::IoError(err) => write!(f, "{}", err),
        }
    }
}

impl From<io::Error> for LzipError {
    fn from(value: io::Error) -> Self {
        LzipError::IoError(value.kind())
    }
}

impl From<stream::Error> for LzipError {
    fn from(value: stream::Error) -> Self {
        LzipError::StreamError(value)
    }
}
